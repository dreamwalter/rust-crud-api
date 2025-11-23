use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

// Stocks 資料庫的 Disposition 模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disposition {
    pub stock_date: Option<NaiveDate>,
    pub market: String,
    pub symbol: i32,
    pub name: String,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDisposition {
    pub stock_date: String,
    pub market: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDisposition {
    pub start: Option<String>,
    pub end: Option<String>,
}

// 通用 API 回應
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}