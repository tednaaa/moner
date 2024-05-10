use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
	pub id: i64,
	pub email: String,
	pub username: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct CreateUserRequest {
	#[validate(email(message = "Must be a valid email address"))]
	pub email: String,
	#[validate(length(min = 6, max = 20, message = "Username must be between 3 and 20 characters"))]
	pub username: String,
	#[validate(length(min = 6, max = 20, message = "Password must be between 3 and 20 characters"))]
	pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivateUserRequest {
	pub user_id: i64,
	#[validate(length(min = 6, max = 6, message = "Code must be 6 characters"))]
	pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserRequest {
	#[validate(length(min = 6, message = "Login must be at least 6 characters"))]
	pub login: String,
	#[validate(length(min = 6, max = 20, message = "Password must be between 3 and 20 characters"))]
	pub password: String,
}
