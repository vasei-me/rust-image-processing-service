use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
    pub username: String,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
    
    pub fn generate_token(&self, user_id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Invalid timestamp")
            .timestamp() as usize;
            
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            username: username.to_string(),
        };
        
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
}