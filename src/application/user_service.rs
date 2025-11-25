use uuid::Uuid;
use crate::domain::UserRepository;
use crate::domain::User;
use crate::core::error::ServiceError;
use crate::core::auth::AuthService;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    pub user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
    
    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, ServiceError> {
        // Check if user already exists
        if self.user_repository.find_by_username(username).await?.is_some() {
            return Err(ServiceError::UserAlreadyExists);
        }
        
        // Hash password
        let password_hash = AuthService::hash_password(password)?;
        
        // Create user
        let user = self.user_repository.create_user(username, &password_hash).await?;
        
        Ok(user)
    }
    
    pub async fn login_user(&self, username: &str, password: &str) -> Result<User, ServiceError> {
        let user = self.user_repository.find_by_username(username).await?
            .ok_or_else(|| ServiceError::AuthenticationError("User not found".to_string()))?;
        
        // Verify password
        if !AuthService::verify_password(password, &user.password_hash)? {
            return Err(ServiceError::AuthenticationError("Invalid password".to_string()));
        }
        
        Ok(user)
    }
    
    pub async fn get_user_profile(&self, user_id: &Uuid) -> Result<User, ServiceError> {
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| ServiceError::ValidationError("User not found".to_string()))?;
        
        Ok(user)
    }
}