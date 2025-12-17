use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod db;
mod handlers;

use db::Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

pub struct AppState {
    db: Mutex<Database>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    log::info!("Starting users service");

    // Initialize database
    let db_path = "/tmp/users.db";
    let db = Database::new(db_path).expect("Failed to initialize database");
    db.init().expect("Failed to initialize database schema");
    
    log::info!("Database initialized at {}", db_path);

    let app_state = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    let bind_addr = "0.0.0.0:9080";
    log::info!("Server listening on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(handlers::health))
            .route("/users", web::get().to(handlers::list_users))
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/{id}", web::get().to(handlers::get_user))
    })
    .bind(bind_addr)?
    .run()
    .await
}
