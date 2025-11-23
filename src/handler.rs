use actix_web::{web, HttpResponse};
use crate::models::{User, CreateUser, UpdateUser, Disposition, ApiResponse};
use crate::repository::{UserRepository, DispositionRepository};
use crate::db::DbPool;

macro_rules! get_conn {
    ($pool:expr, $type:ty) => {
        match $pool.get_conn() {
            Ok(conn) => conn,
            Err(e) => {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<$type>::error(&format!("資料庫連接失敗: {}", e))
                );
            }
        }
    };
}

pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = get_conn!(&pool, User);

    match UserRepository::get_all(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users, "成功獲取所有使用者")),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<User>>::error(&format!("獲取使用者失敗: {}", e))
        ),
    }
}

pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = get_conn!(&pool, User);

    match UserRepository::get_by_id(&mut conn, id) {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user, "成功獲取使用者")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<User>::error(&format!("找不到 ID 為 {} 的使用者", id))
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<User>::error(&format!("獲取使用者失敗: {}", e))
        ),
    }
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> HttpResponse {
    let mut conn = get_conn!(&pool, User);

    match UserRepository::create(&mut conn, &user.into_inner()) {
        Ok(new_user) => HttpResponse::Created().json(ApiResponse::success(new_user, "成功創建使用者")),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Duplicate entry") {
                HttpResponse::BadRequest().json(
                    ApiResponse::<User>::error("電子郵件已存在")
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<User>::error(&format!("創建使用者失敗: {}", error_msg))
                )
            }
        }
    }
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
    user: web::Json<UpdateUser>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = get_conn!(&pool, User);

    match UserRepository::update(&mut conn, id, &user.into_inner()) {
        Ok(Some(updated_user)) => HttpResponse::Ok().json(ApiResponse::success(updated_user, "成功更新使用者")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<User>::error(&format!("找不到 ID 為 {} 的使用者", id))
        ),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Duplicate entry") {
                HttpResponse::BadRequest().json(
                    ApiResponse::<User>::error("電子郵件已存在")
                )
            } else {
                HttpResponse::InternalServerError().json(
                    ApiResponse::<User>::error(&format!("更新使用者失敗: {}", error_msg))
                )
            }
        }
    }
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut conn = get_conn!(&pool, User);

    match UserRepository::delete(&mut conn, id) {
        Ok(true) => HttpResponse::Ok().json(ApiResponse::success(true, "成功刪除使用者")),
        Ok(false) => HttpResponse::NotFound().json(
            ApiResponse::<bool>::error(&format!("找不到 ID 為 {} 的使用者", id))
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<bool>::error(&format!("刪除使用者失敗: {}", e))
        ),
    }
}

pub async fn get_disposition(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = get_conn!(&pool, Disposition);

    match DispositionRepository::get_all(&mut conn) {
        Ok(disposition) => HttpResponse::Ok().json(ApiResponse::success(disposition, "成功獲取所有處置股")),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<Disposition>>::error(&format!("獲取處置股失敗: {}", e))
        ),
    }
}

pub async fn get_disposition_by_symbol(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
) -> HttpResponse {
    let symbol = path.into_inner();
    let mut conn = get_conn!(&pool, Disposition);

    match DispositionRepository::get_by_symbol(&mut conn, symbol) {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user, "成功獲取處置股")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<Disposition>::error(&format!("找不到 ID 為 {} 的處置股", symbol))
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<Disposition>::error(&format!("獲取處置股失敗: {}", e))
        ),
    }
}
