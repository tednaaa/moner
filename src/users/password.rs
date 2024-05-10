use anyhow::{anyhow, Context, Result};
use tokio::task;

use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub async fn hash(password: String) -> Result<String> {
	task::spawn_blocking(move || {
		let salt = SaltString::generate(rand::thread_rng());
		Ok(Argon2::default()
			.hash_password(password.as_bytes(), &salt)
			.map_err(|error| anyhow!(error).context("failed to hash password"))?
			.to_string())
	})
	.await
	.context("panic in password hash()")?
}

pub async fn verify(password: String, hash: String) -> Result<bool> {
	task::spawn_blocking(move || {
		let hash = PasswordHash::new(&hash).map_err(|error| anyhow!(error).context("password hash invalid"))?;
		let result = Argon2::default().verify_password(password.as_bytes(), &hash);

		match result {
			Ok(()) => Ok(true),
			Err(password_hash::Error::Password) => Ok(false),
			Err(error) => Err(anyhow!(error).context("failed to verify password")),
		}
	})
	.await
	.context("panic in password verify()")?
}
