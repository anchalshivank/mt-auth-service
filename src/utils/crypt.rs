use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

/// Hashes a password using bcrypt
pub async fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    // DEFAULT_COST is the recommended cost for bcrypt
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

/// Validates if the provided password matches the hashed password
pub async fn is_valid(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap_or(false)
}
