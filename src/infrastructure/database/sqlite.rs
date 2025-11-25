use sqlx::SqlitePool;
use sqlx::Row;
use uuid::Uuid;
use crate::domain::{UserRepository, user::User};
use crate::core::error::ServiceError;

#[derive(Clone)]  // مطمئن شوید این خط وجود دارد
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<(), ServiceError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY NOT NULL,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to create users table: {}", e))
        })?;

        println!("✅ Users table created or already exists");
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, ServiceError> {
        let id = Uuid::new_v4().to_string();

        let _result = sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(username)
        .bind(password_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.message().contains("UNIQUE constraint failed") {
                    return ServiceError::UserAlreadyExists;
                }
            }
            ServiceError::DatabaseError(format!("Failed to create user: {}", e))
        })?;

        let user_row = sqlx::query(
            r#"
            SELECT id, username, password_hash, created_at 
            FROM users WHERE id = ?
            "#,
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to fetch created user: {}", e))
        })?;

        let user = User {
            id: user_row.get("id"),
            username: user_row.get("username"),
            password_hash: user_row.get("password_hash"),
            created_at: user_row.get("created_at"),
        };

        println!("✅ User created successfully: {}", username);
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ServiceError> {
        let user_row = sqlx::query(
            r#"
            SELECT id, username, password_hash, created_at 
            FROM users WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to find user by username: {}", e))
        })?;

        match user_row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("username"),
                    password_hash: row.get("password_hash"),
                    created_at: row.get("created_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, ServiceError> {
        let id_str = id.to_string();
        
        let user_row = sqlx::query(
            r#"
            SELECT id, username, password_hash, created_at 
            FROM users WHERE id = ?
            "#,
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to find user by id: {}", e))
        })?;

        match user_row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("username"),
                    password_hash: row.get("password_hash"),
                    created_at: row.get("created_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn delete_user(&self, id: &Uuid) -> Result<bool, ServiceError> {
        let id_str = id.to_string();
        
        let result = sqlx::query(
            "DELETE FROM users WHERE id = ?",
        )
        .bind(&id_str)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Failed to delete user: {}", e))
        })?;

        Ok(result.rows_affected() > 0)
    }
}