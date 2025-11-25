use image::imageops;
use std::io::Cursor;
use crate::domain::image::{Image, ImageTransformation, ImageRepository};
use crate::core::error::ServiceError;
use tokio::fs;

pub struct ImageProcessor;

impl ImageProcessor {
    pub async fn process_image(
        &self,
        image_data: &[u8],
        transformations: &ImageTransformation,
    ) -> Result<Vec<u8>, ServiceError> {
        let mut img = image::load_from_memory(image_data)
            .map_err(|e| ServiceError::ValidationError(format!("Invalid image: {}", e)))?;
        
        // Apply transformations
        if let Some(resize) = &transformations.resize {
            img = img.resize(resize.width, resize.height, imageops::FilterType::Lanczos3);
        }
        
        if let Some(crop) = &transformations.crop {
            img = img.crop(crop.x, crop.y, crop.width, crop.height);
        }
        
        if let Some(rotate) = &transformations.rotate {
            if *rotate == 90.0 {
                img = img.rotate90();
            } else if *rotate == 180.0 {
                img = img.rotate180();
            } else if *rotate == 270.0 {
                img = img.rotate270();
            }
        }
        
        if let Some(filters) = &transformations.filters {
            if filters.grayscale {
                img = img.grayscale();
            }
            if let Some(blur) = &filters.blur {
                img = img.blur(*blur);
            }
        }
        
        // Change format - use the new API
        let format = transformations.format.as_deref().unwrap_or("jpeg");
        let mut buffer = Cursor::new(Vec::new());
        
        match format.to_lowercase().as_str() {
            "jpeg" | "jpg" => {
                // For newer versions of image crate
                img.write_to(&mut buffer, image::ImageFormat::Jpeg)
                    .map_err(|e| ServiceError::ValidationError(format!("Failed to encode image: {}", e)))?;
            }
            "png" => {
                img.write_to(&mut buffer, image::ImageFormat::Png)
                    .map_err(|e| ServiceError::ValidationError(format!("Failed to encode image: {}", e)))?;
            }
            _ => return Err(ServiceError::ValidationError("Unsupported format".to_string())),
        }
        
        Ok(buffer.into_inner())
    }
}

#[derive(Clone)]
pub struct ImageService<R: ImageRepository> {
    image_repository: R,
    storage_path: String,
}

impl<R: ImageRepository> ImageService<R> {
    pub fn new(image_repository: R, storage_path: String) -> Self {
        let _ = std::fs::create_dir_all(&storage_path);
        Self {
            image_repository,
            storage_path,
        }
    }
    
    pub async fn upload_image(
        &self,
        user_id: &str,
        filename: &str,
        image_data: &[u8],
    ) -> Result<Image, ServiceError> {
        let image_id = uuid::Uuid::new_v4().to_string();
        let storage_filename = format!("{}_{}", image_id, filename);
        let storage_path = format!("{}/{}", self.storage_path, storage_filename);
        
        // Save file
        fs::write(&storage_path, image_data).await
            .map_err(|e| ServiceError::DatabaseError(format!("Failed to save image file: {}", e)))?;
        
        // Create database record
        let image = Image {
            id: image_id,
            user_id: user_id.to_string(),
            filename: storage_filename,
            original_filename: filename.to_string(),
            file_size: image_data.len() as i64,
            mime_type: "image/jpeg".to_string(),
            storage_path: storage_path.clone(),
            created_at: None,
        };
        
        self.image_repository.create_image(&image).await
    }
    
    pub async fn get_image(&self, image_id: &str, user_id: &str) -> Result<(Image, Vec<u8>), ServiceError> {
        let image = self.image_repository.find_by_id(image_id).await?
            .ok_or(ServiceError::ValidationError("Image not found".to_string()))?;
        
        if image.user_id != user_id {
            return Err(ServiceError::ValidationError("Access denied".to_string()));
        }
        
        let image_data = fs::read(&image.storage_path).await
            .map_err(|e| ServiceError::DatabaseError(format!("Failed to read image file: {}", e)))?;
        
        Ok((image, image_data))
    }
    
    pub async fn list_images(&self, user_id: &str, page: i64, limit: i64) -> Result<Vec<Image>, ServiceError> {
        self.image_repository.find_by_user_id(user_id, page, limit).await
    }
    
    pub async fn transform_image(
        &self,
        image_id: &str,
        user_id: &str,
        transformations: ImageTransformation,
    ) -> Result<Vec<u8>, ServiceError> {
        let (_image, original_data) = self.get_image(image_id, user_id).await?;
        
        let processor = ImageProcessor;
        processor.process_image(&original_data, &transformations).await
    }
    pub async fn delete_image(&self, image_id: &str, user_id: &str) -> Result<bool, ServiceError> {
    // First verify the image exists and belongs to the user
    let _ = self.get_image(image_id, user_id).await?;
    
    // Delete from database
    self.image_repository.delete_image(image_id, user_id).await
}
}