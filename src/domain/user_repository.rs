use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::User;
use crate::core::error::ServiceError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, ServiceError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ServiceError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, ServiceError>;
    async fn delete_user(&self, id: &Uuid) -> Result<bool, ServiceError>;
}