// src/utils/hashing.rs

use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|_| "Error hashing password".to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, String> {
    verify(password, hashed_password).map_err(|_| "Error verifying password".to_string())
}
