use crate::domain::UserRepository;
use crate::domain::user::{User, CreateUserRequest, LoginRequest, AuthResponse, UserResponse};
use crate::core::error::ServiceError;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: String,
    exp: usize,
}

#[derive(Clone)]  // این خط رو اضافه کن
pub struct AuthService<R: UserRepository> {
    user_repository: R,
    jwt_secret: String,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_repository: R, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn register(&self, request: CreateUserRequest) -> Result<AuthResponse, ServiceError> {
        // Check if user exists
        if self.user_repository.find_by_username(&request.username).await?.is_some() {
            return Err(ServiceError::UserAlreadyExists);
        }

        // Hash password
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|_| ServiceError::HashingError)?;

        // Create user
        let user = self.user_repository.create_user(&request.username, &password_hash).await?;

        // Generate token
        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, ServiceError> {
        let user = self.user_repository.find_by_username(&request.username)
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        // Verify password
        if !verify(&request.password, &user.password_hash)
            .map_err(|_| ServiceError::InvalidCredentials)? {
            return Err(ServiceError::InvalidCredentials);
        }

        // Generate token
        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    fn generate_token(&self, user: &User) -> Result<String, ServiceError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Invalid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.username.clone(),
            user_id: user.id.clone(),
            exp: expiration as usize,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.jwt_secret.as_ref()))
            .map_err(|_| ServiceError::TokenCreationError)
    }

    pub fn validate_token(&self, token: &str) -> Result<Uuid, ServiceError> {
        use jsonwebtoken::{decode, DecodingKey, Validation};
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| ServiceError::InvalidToken)?;

        Uuid::parse_str(&token_data.claims.user_id)
            .map_err(|_| ServiceError::InvalidToken)
    }
}