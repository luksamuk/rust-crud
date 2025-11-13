use crate::api::errors::{ApiError, ApiResult};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn generate_hash(password: &str) -> ApiResult<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::UnprocessableEntity("Cannot generate hash from string".into()))?
        .to_string()
        .as_bytes()
        .to_vec())
}

pub fn check_hash(password: &str, hash: &str) -> ApiResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| ApiError::UnprocessableEntity("Cannot parse password".into()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
