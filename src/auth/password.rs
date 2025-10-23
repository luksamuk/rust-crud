//! This module wraps functions related to password encryption.

use crate::api::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use rocket_anyhow::Result;

/// Generate a hash from a slice. Returns the actual bytes of
/// the hashing process. If the hash cannot be generated, panics.
pub fn generate_hash(password: &str) -> Result<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::new("Cannot generate hash from string"))?
        .to_string()
        .as_bytes()
        .to_vec())
}

/// Check whether a given password matches a given generated hash.
/// Use this function for password authentication.
pub fn check_hash(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|_| AppError::new("Cannot parse password"))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
