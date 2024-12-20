// src/utils/jwt.rs

use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Function to generate a JWT token
pub fn generate_jwt(email: &str) -> Result<String, String> {
    let secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;

    let claims = Claims {
        sub: email.to_string(),
        exp: 10000000000, // Set appropriate expiration time
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).map_err(|_| "Error encoding JWT".to_string())
}
