use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::{AppState, CreateUserRequest};
use crate::db;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "UP"
    }))
}

pub async fn list_users(state: web::Data<AppState>) -> HttpResponse {
    let client = state.client.clone();
    
    // Use web::block to run blocking database operation in a thread pool.
    // This preserves OpenTelemetry eBPF Instrumentation (OBI) trace context
    //  while preventing blocking of the async runtime.
    match web::block(move || {
        let mut conn = client.lock().unwrap();
        db::list_users(&mut conn)
    }).await {
        Ok(Ok(users)) => HttpResponse::Ok().json(users),
        Ok(Err(e)) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to list users"
            }))
        }
        Err(e) => {
            log::error!("Blocking error: {}", e);
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
    let client = state.client.clone();
    
    // Use web::block to run blocking database operation in a thread pool.
    // This preserves OpenTelemetry eBPF Instrumentation (OBI) trace context while preventing blocking of the async runtime.
    match web::block(move || {
        let mut conn = client.lock().unwrap();
        db::get_user(&mut conn, user_id)
    }).await {
        Ok(Ok(Some(user))) => HttpResponse::Ok().json(user),
        Ok(Ok(None)) => HttpResponse::NotFound().json(json!({
            "error": "User not found"
        })),
        Ok(Err(e)) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to get user"
            }))
        }
        Err(e) => {
            log::error!("Blocking error: {}", e);
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
    let username = req.username.trim().to_string();
    
    if username.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Username cannot be empty"
        }));
    }

    let client = state.client.clone();
    
    // Use web::block to run blocking database operation in a thread pool.
    // This preserves OpenTelemetry eBPF Instrumentation (OBI) trace context while preventing blocking of the async runtime.
    match web::block(move || {
        let mut conn = client.lock().unwrap();
        db::create_or_get_user(&mut conn, &username)
    }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(e)) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create user"
            }))
        }
        Err(e) => {
            log::error!("Blocking error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create user"
            }))
        }
    }
}
