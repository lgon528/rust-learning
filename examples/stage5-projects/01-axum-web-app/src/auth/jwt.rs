use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::models::user::Claims;
use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }

    fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }

    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    pub fn generate_token(&self, user_id: &str, username: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // 24 hours expiration

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key())
            .map_err(AppError::Jwt)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key(),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(AppError::Jwt)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let secret = "test_secret_key";
        let jwt_service = JwtService::new(secret);

        let user_id = "123e4567-e89b-12d3-a456-426614174000";
        let username = "testuser";

        // Generate token
        let token = jwt_service.generate_token(user_id, username).unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_invalid_token() {
        let secret = "test_secret_key";
        let jwt_service = JwtService::new(secret);

        let invalid_token = "invalid.jwt.token";
        let result = jwt_service.validate_token(invalid_token);
        assert!(result.is_err());
    }
}
