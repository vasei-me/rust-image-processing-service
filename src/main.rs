mod core;
mod domain;
mod infrastructure;
mod application;
mod api;

use std::net::SocketAddr;
use sqlx::SqlitePool;
use crate::core::jwt::JwtService;
use crate::application::{user_service::UserService, image_service::ImageService};
use crate::infrastructure::{SqliteUserRepository, SqliteImageRepository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Image Processing Service...");

    // Connect to database
    let database_url = "sqlite::memory:"; // استفاده از memory برای تست
    let pool = SqlitePool::connect(database_url).await?;
    println!("✅ Connected to database successfully");

    // Create repositories
    let user_repository = SqliteUserRepository::new(pool.clone());
    let image_repository = SqliteImageRepository::new(pool.clone());

    // Create tables
    user_repository.create_table().await?;
    image_repository.create_table().await?;
    println!("📋 Database tables created");

    // Create upload directory
    let storage_path = "./uploads".to_string();
    tokio::fs::create_dir_all(&storage_path).await?;
    println!("📁 Created uploads directory");

    // Create services
    let user_service = UserService::new(user_repository);
    let image_service = ImageService::new(image_repository, storage_path);
    let jwt_service = JwtService::new("your-super-secret-jwt-key".to_string());

    // Create router
    let app = api::routes::create_router(user_service, image_service, jwt_service);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🌐 Server listening on {}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app
    ).await?;

    Ok(())
}