mod models;
mod db;
mod repository;
mod handler;
use actix_web::HttpResponse;
use actix_cors::Cors;
use crate::models::ApiResponse;

use actix_web::{web, App, HttpServer};
use handler::*;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("OK", "服務正常運行"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化資料庫連接池
    let pool = match db::create_pool() {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ 無法建立資料庫連接池: {}", e);
            std::process::exit(1);
        }
    };

    println!("🚀 啟動 Rust CRUD API 伺服器...");

    HttpServer::new(move || {  
        // 配置 CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5174")  // 你的 React 開發服務器
            .allowed_origin("http://localhost:3000")  // Create React App 默認端口
            .allowed_origin("http://127.0.0.1:5174")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                "Content-Type",
                "Authorization",
                "Accept",
            ])
            .supports_credentials()
            .max_age(3600);
            
        // to(handler)
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .route("/health", web::get().to(health_check))  // 健康檢查
            .route("/user", web::get().to(get_user))
            .route("/user/{id}", web::get().to(get_user_by_id))
            .route("/user", web::post().to(create_user))
            .route("/user/{id}", web::put().to(update_user))
            .route("/user/{id}", web::delete().to(delete_user))
            .route("/disposition", web::get().to(get_disposition))
            .route("/disposition/{symbol}", web::get().to(get_disposition_by_symbol))
            .route("/disposition", web::post().to(create_disposition))
            .route("/disposition/{symbol}", web::put().to(update_disposition))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}