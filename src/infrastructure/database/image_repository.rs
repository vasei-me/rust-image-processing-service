use sqlx::SqlitePool;
use crate::domain::image::{Image, ImageRepository};
use crate::core::error::ServiceError;

#[derive(Clone)]  // اضافه کردن این خط
pub struct SqliteImageRepository {
    pool: SqlitePool,
}

impl SqliteImageRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<(), ServiceError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS images (
                id TEXT PRIMARY KEY NOT NULL,
                user_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                original_filename TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                storage_path TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to create images table: {}", e))
        })?;

        println!("✅ Images table created or already exists");
        Ok(())
    }
}

#[async_trait::async_trait]
impl ImageRepository for SqliteImageRepository {
    async fn create_image(&self, image: &Image) -> Result<Image, ServiceError> {
        let _result = sqlx::query(
            r#"
            INSERT INTO images (id, user_id, filename, original_filename, file_size, mime_type, storage_path)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&image.id)
        .bind(&image.user_id)
        .bind(&image.filename)
        .bind(&image.original_filename)
        .bind(image.file_size)
        .bind(&image.mime_type)
        .bind(&image.storage_path)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to create image: {}", e))
        })?;

        let created_image = sqlx::query_as::<_, Image>(
            r#"
            SELECT id, user_id, filename, original_filename, file_size, mime_type, storage_path, created_at
            FROM images WHERE id = ?
            "#,
        )
        .bind(&image.id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to fetch created image: {}", e))
        })?;

        Ok(created_image)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Image>, ServiceError> {
        let image = sqlx::query_as::<_, Image>(
            r#"
            SELECT id, user_id, filename, original_filename, file_size, mime_type, storage_path, created_at
            FROM images WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to find image by id: {}", e))
        })?;

        Ok(image)
    }

    async fn find_by_user_id(&self, user_id: &str, page: i64, limit: i64) -> Result<Vec<Image>, ServiceError> {
        let offset = (page - 1) * limit;
        let images = sqlx::query_as::<_, Image>(
            r#"
            SELECT id, user_id, filename, original_filename, file_size, mime_type, storage_path, created_at
            FROM images WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to find images by user_id: {}", e))
        })?;

        Ok(images)
    }

    async fn delete_image(&self, id: &str, user_id: &str) -> Result<bool, ServiceError> {
        let result = sqlx::query(
            "DELETE FROM images WHERE id = ? AND user_id = ?",
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to delete image: {}", e))
        })?;

        Ok(result.rows_affected() > 0)
    }
}