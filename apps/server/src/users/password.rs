use anyhow::{anyhow, Result};

use argon2::password_hash::SaltString;
use argon2::{password_hash::rand_core::OsRng, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash(password: &str) -> Result<String> {
	let salt = SaltString::generate(&mut OsRng);
	Ok(Argon2::default()
		.hash_password(password.as_bytes(), &salt)
		.map_err(|_| anyhow!("failed to hash password"))?
		.to_string())
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
	let parsed_hash = PasswordHash::new(hash).map_err(|_| anyhow!("invalid password hash"))?;
	Ok(Argon2::default()
		.verify_password(password.as_bytes(), &parsed_hash)
		.is_ok())
}
