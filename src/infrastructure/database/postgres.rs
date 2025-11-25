use sqlx::PgPool;
use crate::domain::{UserRepository, ImageRepository, user::User, image::Image};
use crate::core::error::ServiceError;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, ServiceError> {
        let id = Uuid::new_v4();
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, password_hash, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, username, password_hash, created_at
            "#,
            id,
            username,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ServiceError::DatabaseError)?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_at 
            FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ServiceError::DatabaseError)?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_at 
            FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ServiceError::DatabaseError)?;

        Ok(user)
    }
}