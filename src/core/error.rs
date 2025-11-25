use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Image processing error: {0}")]
    ImageProcessingError(String),
    
    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::ImageProcessingError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for ServiceError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ServiceError::ValidationError(err.to_string())
    }
}

// Implement conversion from ServiceError to axum response
impl axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            ServiceError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::UserAlreadyExists => axum::http::StatusCode::CONFLICT,
            ServiceError::ValidationError(_) => axum::http::StatusCode::BAD_REQUEST,
            ServiceError::AuthenticationError(_) => axum::http::StatusCode::UNAUTHORIZED,
            ServiceError::ImageProcessingError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
        };
        
        (status, self.to_string()).into_response()
    }
}