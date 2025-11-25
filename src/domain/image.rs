use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub storage_path: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTransformation {
    pub resize: Option<Resize>,
    pub crop: Option<Crop>,
    pub rotate: Option<f32>,
    pub format: Option<String>,
    pub filters: Option<Filters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filters {
    pub grayscale: bool,
    pub sepia: bool,
    pub blur: Option<f32>,
}

// Repository trait برای تصاویر
#[async_trait::async_trait]
pub trait ImageRepository: Send + Sync {
    async fn create_image(&self, image: &Image) -> Result<Image, crate::core::error::ServiceError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Image>, crate::core::error::ServiceError>;
    async fn find_by_user_id(&self, user_id: &str, page: i64, limit: i64) -> Result<Vec<Image>, crate::core::error::ServiceError>;
    async fn delete_image(&self, id: &str, user_id: &str) -> Result<bool, crate::core::error::ServiceError>;
}