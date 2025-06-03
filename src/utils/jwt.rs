use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
}

impl Claims {
    pub fn new(user_id: String, email: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours
        
        Claims {
            sub: user_id,
            email,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_jwt(user_id: String, email: String, secret: &str) -> Result<String> {
    let claims = Claims::new(user_id, email);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
