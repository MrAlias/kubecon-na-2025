use actix_web::{web, App, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};
use std::sync::{Arc, Mutex};

mod db;
mod handlers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

// Using a single synchronous postgres client wrapped in Arc<Mutex> instead of
// async connection pooling. This design choice ensures proper trace context
// propagation when using OpenTelemetry eBPF Instrumentation (OBI).
// Async database operations can break tracing context when spawning tasks.
pub struct AppState {
    client: Arc<Mutex<Client>>,
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
        .unwrap_or_else(|_| "postgresql://postgres@users-db.default.svc.cluster.local:5432/main".to_string());
    
    log::info!("Connecting to database at {}", database_url);

    // Create single synchronous connection (no pooling) in a blocking thread.
    // This maintains proper trace context propagation with OpenTelemetry eBPF Instrumentation (OBI).
    // Async connection handling breaks the trace context chain.
    // We initialize in a blocking thread to avoid runtime nesting issues.
    let client = std::thread::spawn(move || {
        let mut client = Client::connect(&database_url, NoTls)
            .expect("Failed to connect to database");

        // Initialize database schema
        db::init_database(&mut client)
            .expect("Failed to initialize database schema");
        
        client
    })
    .join()
    .expect("Failed to join database initialization thread");
    
    log::info!("Database initialized successfully");

    let app_state = web::Data::new(AppState {
        client: Arc::new(Mutex::new(client)),
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
