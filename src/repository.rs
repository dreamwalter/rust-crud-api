use crate::models::{User, CreateUser, UpdateUser, Disposition, CreateDisposition, UpdateDisposition};
use anyhow::Result;
use mysql::{prelude::*, PooledConn, Value};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

pub struct UserRepository;

// 提取重複的 datetime 解析邏輯
pub fn parse_datetime(val: Value) -> Option<NaiveDateTime> {
    match val {
        Value::Date(y, m, d, h, min, s, _) => {
            NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32)
                .and_then(|date| NaiveTime::from_hms_opt(h as u32, min as u32, s as u32)
                    .map(|time| NaiveDateTime::new(date, time)))
        },
        _ => None,
    }
}     

impl UserRepository {
    pub fn get_all(conn: &mut PooledConn) -> Result<Vec<User>> {
        let query = "SELECT id, name, email, created_at, updated_at FROM user";
        
        let rows: Vec<(u32, String, String, Value, Value)> = conn.exec(query, ())?;
      
        let user: Vec<User> = rows.into_iter().map(|(id, name, email, created_val, updated_val)| {
            let created_at = parse_datetime(created_val);
            let updated_at = parse_datetime(updated_val);
            User { id, name, email, created_at, updated_at }
        }).collect();
      
        Ok(user)
    }

    pub fn get_by_id(conn: &mut PooledConn, id: u32) -> Result<Option<User>> {
        let query = "SELECT id, name, email, created_at, updated_at FROM user WHERE id = ?";
        
        let row_opt: Option<(u32, String, String, Value, Value)> = conn.exec_first(query, (id,))?;
    
        if let Some((id, name, email, created_val, updated_val)) = row_opt {
            let created_at = parse_datetime(created_val);
            let updated_at = parse_datetime(updated_val);    
            Ok(Some(User { id, name, email, created_at, updated_at }))
        } else {
            Ok(None)
        }
    }

    pub fn create(conn: &mut PooledConn, user: &CreateUser) -> Result<User> {
        let query = "INSERT INTO user (name, email) VALUES (?, ?)";
        conn.exec_drop(query, ( &user.name, &user.email ))?;

        let user_id = conn.last_insert_id();
        if let Some(user) = Self::get_by_id(conn, user_id as u32)? {
            Ok(user)
        } else {
            anyhow::bail!("無法獲取新創建的使用者")
        }
    }

    pub fn update(conn: &mut PooledConn, id: u32, user: &UpdateUser) -> Result<Option<User>> {
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(name) = &user.name {
            updates.push("name = ?");
            params.push(name.clone());
        }

        if let Some(email) = &user.email {
            updates.push("email = ?");
            params.push(email.clone());
        }

        if updates.is_empty() {
            return Self::get_by_id(conn, id);
        }

        let query = format!("UPDATE user SET {} WHERE id = ?", updates.join(", "));
        params.push(id.to_string());

        conn.exec_drop(&query, params)?;

        Self::get_by_id(conn, id)
    }

    pub fn delete(conn: &mut PooledConn, id: u32) -> Result<bool> {
        let query = "DELETE FROM user WHERE id = ?";
        
        let result = conn.exec_iter(query, (id,))?;
        
        let affected_rows = result.affected_rows();

        Ok(affected_rows > 0)
    }
}

pub struct DispositionRepository;

pub fn parse_date(val: Value) -> Option<NaiveDate> {
    match val {
        Value::Date(y, m, d, _, _, _, _) => {
            NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32)
        },
        _ => None,
    }
}    

impl DispositionRepository {
    pub fn get_all(conn: &mut PooledConn) -> Result<Vec<Disposition>> {
        let query = "SELECT stock_date, market, symbol, name, start, end, created_at, updated_at FROM s_disposition";
        
        let rows: Vec<(Value, String, i32, String, Value, Value, Value, Value)> = conn.exec(query, ())?;
      
        let disposition: Vec<Disposition> = rows.into_iter().map(|(stock_date_val, market, symbol, name, start_val, end_val, created_val, updated_val)| {
            let stock_date = parse_date(stock_date_val);
            let start = parse_date(start_val);
            let end = parse_date(end_val);
            let created_at = parse_datetime(created_val);
            let updated_at = parse_datetime(updated_val);
            Disposition { stock_date, market, symbol, name, start, end, created_at, updated_at }
        }).collect();
      
        Ok(disposition)
    }

    pub fn get_by_symbol(conn: &mut PooledConn, symbol: i32) -> Result<Option<Disposition>> {
        let query = "SELECT stock_date, market, symbol, name, start, end, created_at, updated_at FROM s_disposition WHERE symbol = ? ORDER BY end DESC LIMIT 1";

        let row_opt: Option<(Value, String, i32, String, Value, Value, Value, Value)> = conn.exec_first(query, (symbol,))?;
    
        if let Some((stock_date_val, market, symbol, name, start_val, end_val, created_val, updated_val)) = row_opt {
            let stock_date = parse_date(stock_date_val);
            let start = parse_date(start_val);
            let end = parse_date(end_val);
            let created_at = parse_datetime(created_val);
            let updated_at = parse_datetime(updated_val);    
            Ok(Some(Disposition { stock_date, market, symbol, name, start, end, created_at, updated_at }))
        } else {
            Ok(None)
        }
    }

    pub fn create(conn: &mut PooledConn, disposition: &CreateDisposition) -> Result<Disposition> {
        let query = "INSERT INTO s_disposition (stock_date, market, symbol, name) VALUES (?, ?, ?, ?)";
        let symbol_num: i32 = disposition.symbol.parse().map_err(|e| {
            anyhow::anyhow!("無效的股票代碼格式 '{}': {}", disposition.symbol, e)
        })?;
        conn.exec_drop(query, ( &disposition.stock_date, &disposition.market, symbol_num, &disposition.name ))?;

        if let Some(disposition) = Self::get_by_symbol(conn, symbol_num)? {
            Ok(disposition)
        } else {
            anyhow::bail!("無法獲取新創建的處置股")
        }
    }

    pub fn update(conn: &mut PooledConn, symbol: i32, disposition: &UpdateDisposition) -> Result<Option<Disposition>> {
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(start) = &disposition.start {
            updates.push("start = ?");
            params.push(start.clone());
        }

        if let Some(end) = &disposition.end {
            updates.push("end = ?");
            params.push(end.clone());
        }

        if updates.is_empty() {
            return Self::get_by_symbol(conn, symbol);
        }

        let query = format!("UPDATE s_disposition SET {} WHERE symbol = ? ORDER BY end DESC LIMIT 1", updates.join(", "));
        params.push(symbol.to_string());

        conn.exec_drop(&query, params)?;

        Self::get_by_symbol(conn, symbol)
    }

}
