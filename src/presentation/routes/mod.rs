// DEPRECATED: This module uses actix-web which conflicts with axum
// Keeping for reference only - use api module instead
#![allow(dead_code)]
#![allow(unused_imports)]

/*
use actix_web::{web, HttpResponse};
use std::collections::HashMap;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/health", web::get().to(health_check))
    );
}

async fn register(body: web::Json<serde_json::Value>) -> HttpResponse {
    let username = body.get("username").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    let mut response = HashMap::new();
    response.insert("message", "User registration - REAL implementation coming soon");
    response.insert("status", "success");
    response.insert("username", username);
    response.insert("note", "Database connected, AuthService integration in progress");
    
    HttpResponse::Ok().json(response)
}

async fn login(body: web::Json<serde_json::Value>) -> HttpResponse {
    let username = body.get("username").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    let mut response = HashMap::new();
    response.insert("message", "User login - REAL implementation coming soon");
    response.insert("status", "success");
    response.insert("username", username);
    response.insert("note", "JWT authentication integration in progress");
    
    HttpResponse::Ok().json(response)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Image Processing Service is running 🚀",
        "database": "SQLite (ACTIVE & CONNECTED)"
    }))
}
*/