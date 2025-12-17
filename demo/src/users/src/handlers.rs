use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::{AppState, User, CreateUserRequest};

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "UP"
    }))
}

pub async fn list_users(state: web::Data<AppState>) -> HttpResponse {
    let db = state.db.lock().unwrap();
    
    match db.list_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to list users"
            }))
        }
    }
}

pub async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let db = state.db.lock().unwrap();
    
    match db.get_user(user_id) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "User not found"
        })),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to get user"
            }))
        }
    }
}

pub async fn create_user(
    state: web::Data<AppState>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let username = req.username.trim();
    
    if username.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Username cannot be empty"
        }));
    }

    let db = state.db.lock().unwrap();
    
    match db.create_or_get_user(username) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create user"
            }))
        }
    }
}
