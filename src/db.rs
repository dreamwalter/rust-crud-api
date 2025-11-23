use mysql::*;
use mysql::prelude::*;
use std::sync::Arc;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

// 修正：應該使用 Arc<mysql::Pool> 而不是 Arc<PooledConn>
pub type DbPool = Arc<mysql::Pool>;

pub fn create_pool() -> Result<DbPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    // println!("{}", &database_url1);

    // 創建連接池時可以設定更多選項 
    // database_url是文字，但 &str 才是我們要的，所以要加上 &
    let opts = Opts::from_url(&database_url)
        .map_err(|e| anyhow::anyhow!("解析資料庫 URL 失敗: {}", e))?;
    
    let pool = Pool::new(opts)?;

    // 測試連接
    let mut conn = pool.get_conn()?;
    conn.query_drop("SELECT 1")?;
    
    println!("✅ 成功連接到 MySQL 資料庫");
    
    // 使用 Arc 包裝連接池，以便在多線程間安全共享
    Ok(Arc::new(pool))
}
