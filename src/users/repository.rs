use std::sync::Arc;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;

use crate::database::Database;

use super::dtos::{PublicUserResponse, UserResponse};

#[derive(Clone)]
pub struct UsersRepostory {
	pub(crate) database: Arc<Database>,
}

impl UsersRepostory {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			database: Arc::clone(database),
		}
	}

	pub async fn find_user_by_id(&self, id: &i64) -> anyhow::Result<User> {
		let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
			.fetch_one(&*self.database.pool)
			.await
			.map_err(|error| match error {
				sqlx::Error::RowNotFound => anyhow!("User not found"),
				_ => anyhow!(error).context("Failed to find user by id"),
			})?;

		Ok(user)
	}

	pub async fn create_user(&self, email: &str, username: &str, password: &str) -> anyhow::Result<User> {
		let user = sqlx::query_as!(
			User,
			r#"
				INSERT INTO users (email, username, password)
				VALUES ($1, $2, $3) RETURNING *
			"#,
			email,
			username,
			password,
		)
		.fetch_one(&*self.database.pool)
		.await
		.map_err(|error| match error {
			sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_email_key") => {
				anyhow!("Email already exists")
			}
			sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
				anyhow!("Username already exists")
			}
			_ => anyhow!(error).context("Failed to create user"),
		})?;

		Ok(user)
	}

	pub async fn verify_user(&self, user_id: &i64) -> anyhow::Result<User> {
		let user = sqlx::query_as!(
			User,
			r#"
				UPDATE users SET is_verified = true
				WHERE id = $1 RETURNING *
			"#,
			user_id
		)
		.fetch_one(&*self.database.pool)
		.await
		.map_err(|error| anyhow!(error).context("Failed to verify user"))?;

		Ok(user)
	}

	pub async fn delete_unverified_user(&self, email: &str) -> anyhow::Result<PgQueryResult> {
		let query_result = sqlx::query!("DELETE FROM users WHERE email = $1 AND is_verified = false", email)
			.execute(&*self.database.pool)
			.await
			.map_err(|error: sqlx::Error| anyhow!(error).context("Failed to delete unverified user"))?;

		Ok(query_result)
	}

	pub async fn delete_user_by_id(&self, user_id: &i64) -> anyhow::Result<PgQueryResult> {
		let query_result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
			.execute(&*self.database.pool)
			.await
			.map_err(|error: sqlx::Error| anyhow!(error).context("Failed to delete user by id"))?;

		Ok(query_result)
	}

	pub async fn find_user_by_login(&self, login: &str) -> anyhow::Result<User> {
		let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 OR username = $1", login)
			.fetch_one(&*self.database.pool)
			.await
			.map_err(|error| anyhow!(error).context("Failed to find user by login"))?;

		Ok(user)
	}

	pub async fn find_user_by_email(&self, email: &str) -> anyhow::Result<User> {
		let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
			.fetch_one(&*self.database.pool)
			.await
			.map_err(|error| anyhow!(error).context("Failed to find user by email"))?;

		Ok(user)
	}

	pub async fn find_user_by_username(&self, username: &str) -> anyhow::Result<User> {
		let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
			.fetch_one(&*self.database.pool)
			.await
			.map_err(|error| anyhow!(error).context("Failed to find user by username"))?;

		Ok(user)
	}

	pub async fn change_password(&self, user_id: &i64, new_password: &str) -> anyhow::Result<PgQueryResult> {
		let query_result = sqlx::query!("UPDATE users SET password = $1 WHERE id = $2", new_password, user_id)
			.execute(&*self.database.pool)
			.await
			.map_err(|error: sqlx::Error| anyhow!(error).context("Failed to change password"))?;

		Ok(query_result)
	}
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct User {
	pub id: i64,
	pub email: String,
	pub username: String,
	pub password: String,
	pub is_verified: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			email: user.email,
			username: user.username,
			created_at: user.created_at,
			updated_at: user.updated_at,
		}
	}
}

impl From<User> for PublicUserResponse {
	fn from(user: User) -> Self {
		Self {
			email: user.email,
			username: user.username,
		}
	}
}
