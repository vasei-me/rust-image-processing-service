use bcrypt::{hash, verify, DEFAULT_COST};
use crate::core::error::ServiceError;

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, ServiceError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| ServiceError::ValidationError(format!("Failed to hash password: {}", e)))
    }
    
    pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, ServiceError> {
        verify(password, hashed_password)
            .map_err(|e| ServiceError::ValidationError(format!("Failed to verify password: {}", e)))
    }
}