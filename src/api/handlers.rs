use axum::{
    extract::{State, Multipart, Path, Query},
    response::Json,
    http::StatusCode,
};
use serde::Deserialize;
use crate::application::user_service::UserService;
use crate::application::image_service::ImageService;
use crate::domain::user_repository::UserRepository;
use crate::domain::image::{ImageRepository, ImageTransformation};
use crate::core::error::ServiceError;
use crate::core::jwt::JwtService;

// DTOs برای درخواست‌ها
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// پاسخ‌ها
#[derive(serde::Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub created_at: Option<String>,
}

#[derive(serde::Serialize, Clone)]
pub struct ImageResponse {
    pub id: String,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ImageListResponse {
    pub images: Vec<ImageResponse>,
    pub total: usize,
    pub page: i64,
    pub limit: i64,
}

// Generic handlers
pub async fn register<UR: UserRepository>(
    State((user_service, jwt_service)): State<(UserService<UR>, JwtService)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ServiceError> {
    let user = user_service.register_user(&payload.username, &payload.password).await?;
    
    let token = jwt_service.generate_token(&user.id, &user.username)
        .map_err(|e| ServiceError::ValidationError(format!("Failed to generate token: {}", e)))?;
    
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
    };
    
    Ok(Json(AuthResponse {
        user: user_response,
        token,
    }))
}

pub async fn login<UR: UserRepository>(
    State((user_service, jwt_service)): State<(UserService<UR>, JwtService)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ServiceError> {
    let user = user_service.login_user(&payload.username, &payload.password).await?;
    
    let token = jwt_service.generate_token(&user.id, &user.username)
        .map_err(|e| ServiceError::ValidationError(format!("Failed to generate token: {}", e)))?;
    
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
    };
    
    Ok(Json(AuthResponse {
        user: user_response,
        token,
    }))
}

// Simple test handlers
pub async fn upload_image_simple<IR: ImageRepository>(
    State(image_service): State<ImageService<IR>>,
    mut multipart: Multipart,
) -> Result<Json<ImageResponse>, ServiceError> {
    let mut image_data = Vec::new();
    let mut filename = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ServiceError::ValidationError(format!("Multipart error: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "image" {
            filename = field.file_name().map(|f| f.to_string());
            let data = field.bytes().await.map_err(|e| {
                ServiceError::ValidationError(format!("Failed to read image data: {}", e))
            })?;
            image_data = data.to_vec();
        }
    }

    if image_data.is_empty() {
        return Err(ServiceError::ValidationError("No image data provided".to_string()));
    }

    let filename = filename.unwrap_or_else(|| "unknown.jpg".to_string());
    
    let user_id = "ad808fc5-a806-481a-ac15-3ea8fbdc66da";
    
    let image = image_service.upload_image(user_id, &filename, &image_data).await?;

    let response = ImageResponse {
        id: image.id,
        filename: image.filename,
        original_filename: image.original_filename,
        file_size: image.file_size,
        mime_type: "image/jpeg".to_string(),
        created_at: image.created_at,
    };

    Ok(Json(response))
}

pub async fn transform_image_simple<IR: ImageRepository>(
    State(image_service): State<ImageService<IR>>,
    Path(image_id): Path<String>,
    Json(transformations): Json<ImageTransformation>,
) -> Result<axum::response::Response, ServiceError> {
    let user_id = "ad808fc5-a806-481a-ac15-3ea8fbdc66da";
    let processed_image = image_service.transform_image(&image_id, user_id, transformations).await?;
    
    Ok(axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "image/jpeg")
        .body(axum::body::Body::from(processed_image))
        .unwrap())
}

pub async fn get_image_simple<IR: ImageRepository>(
    State(image_service): State<ImageService<IR>>,
    Path(image_id): Path<String>,
) -> Result<axum::response::Response, ServiceError> {
    let user_id = "ad808fc5-a806-481a-ac15-3ea8fbdc66da";
    let (image, image_data) = image_service.get_image(&image_id, user_id).await?;
    
    Ok(axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", &image.mime_type)
        .header("content-disposition", format!("inline; filename=\"{}\"", image.original_filename))
        .body(axum::body::Body::from(image_data))
        .unwrap())
}

pub async fn list_images_simple<IR: ImageRepository>(
    State(image_service): State<ImageService<IR>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ImageListResponse>, ServiceError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10).min(100);
    
    let user_id = "ad808fc5-a806-481a-ac15-3ea8fbdc66da";
    let images = image_service.list_images(user_id, page, limit).await?;
    
    let image_responses: Vec<ImageResponse> = images.into_iter().map(|image| ImageResponse {
        id: image.id,
        filename: image.filename,
        original_filename: image.original_filename,
        file_size: image.file_size,
        mime_type: image.mime_type,
        created_at: image.created_at,
    }).collect();
    
    let total = image_responses.len();
    
    Ok(Json(ImageListResponse {
        images: image_responses,
        total,
        page,
        limit,
    }))
}

// Main handlers (for future use)
#[allow(dead_code)]
pub async fn upload_image() -> &'static str {
    "Upload image endpoint (with auth) - Coming soon"
}

#[allow(dead_code)]
pub async fn transform_image() -> &'static str {
    "Transform image endpoint (with auth) - Coming soon"
}

#[allow(dead_code)]
pub async fn get_image() -> &'static str {
    "Get image endpoint (with auth) - Coming soon"
}

#[allow(dead_code)]
pub async fn list_images() -> &'static str {
    "List images endpoint (with auth) - Coming soon"
}

#[allow(dead_code)]
pub async fn delete_image() -> &'static str {
    "Delete image endpoint (with auth) - Coming soon"
}