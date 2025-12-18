use actix_web::{web, App, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    db: Arc<Database>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    log::info!("Starting users service");

    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "http://users-db.default.svc.cluster.local:8080".to_string());
    
    log::info!("Connecting to database at {}", database_url);

    // Initialize database
    let db = Database::new(&database_url).await
        .expect("Failed to initialize database");
    db.init().await
        .expect("Failed to initialize database schema");
    
    log::info!("Database initialized successfully");

    let app_state = web::Data::new(AppState {
        db: Arc::new(db),
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
