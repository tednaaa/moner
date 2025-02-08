use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	middleware,
	response::{IntoResponse, Response},
	routing::{get, patch, post},
	Extension, Json, Router,
};
use thiserror::Error;
use tower_cookies::{Cookie, Cookies};

use crate::{
	app::{ApiErrorResponse, ApiResult},
	database::Database,
	services::email::EmailService,
	validation::ValidatedJson,
};

use super::{
	auth::{self, authorize_jwt, CurrentUser},
	dtos::{ChangePasswordRequest, CreateUserRequest, LoginUserRequest, PublicUserResponse, UserResponse},
	password,
	repository::UsersRepostory,
};

#[derive(Clone)]
pub struct UsersState {
	pub users_repository: UsersRepostory,
	pub email_service: EmailService,
}

impl UsersState {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			users_repository: UsersRepostory::new(database),
			email_service: EmailService::new(),
		}
	}
}

pub fn init() -> Router<UsersState> {
	Router::new()
		.route("/users/me", get(get_me_route))
		.route("/users/password/change", patch(password_change_route))
		.route_layer(middleware::from_fn(auth::middleware))
		.route("/users/register", post(register_user_route))
		.route("/users/login", post(login_user_route))
		.route("/users/logout", get(logout_user_route))
		.route("/users/{username}", get(get_public_user_route))
}

async fn register_user_route(
	State(state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	state
		.users_repository
		.delete_unverified_user(&request.email)
		.await
		.map_err(|_| UsersApiError::FailedToCreateUser())?;

	let password_hash = password::hash(&request.password).map_err(|_| UsersApiError::FailedToCreateUser())?;

	let created_user = state
		.users_repository
		.create_user(&request.email, &request.username, &password_hash)
		.await
		.map_err(|error| match error.to_string().to_lowercase() {
			string if string.contains("email") => UsersApiError::EmailTaken(request.email.clone()),
			string if string.contains("username") => UsersApiError::UsernameTaken(request.username.clone()),
			_ => UsersApiError::FailedToCreateUser(),
		})?;

	Ok((StatusCode::CREATED, Json(UserResponse::from(created_user))))
}

async fn login_user_route(
	cookies: Cookies,
	State(state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<LoginUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	let user = state
		.users_repository
		.find_user_by_login(&request.login)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.login.clone()))?;

	let is_valid_password =
		password::verify(&request.password, &user.password).map_err(|_| UsersApiError::WrongPassword())?;

	if !is_valid_password {
		return Err(UsersApiError::WrongPassword())?;
	}

	authorize_jwt(&cookies, &CurrentUser::from(user.clone())).map_err(|_| UsersApiError::FailedToLoginUser())?;
	Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

async fn logout_user_route(cookies: Cookies) -> ApiResult<()> {
	let access_token_cookie = Cookie::build(("access_token", "")).path("/").http_only(true).build();

	cookies.remove(access_token_cookie);
	Ok((StatusCode::OK, ()))
}

async fn get_me_route(
	Extension(current_user): Extension<CurrentUser>,
	State(UsersState { users_repository, .. }): State<UsersState>,
) -> ApiResult<Json<UserResponse>> {
	let user = users_repository
		.find_user_by_id(&current_user.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(current_user.user_id.to_string()))?;

	if !user.is_verified {
		return Err(UsersApiError::UserNotVerified(current_user.user_id.to_string()))?;
	}

	Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

async fn get_public_user_route(
	Path(username): Path<String>,
	State(state): State<UsersState>,
) -> ApiResult<Json<PublicUserResponse>> {
	let user = state
		.users_repository
		.find_user_by_username(&username)
		.await
		.map_err(|_| UsersApiError::UserNotFound(username.clone()))?;

	Ok((StatusCode::OK, Json(PublicUserResponse::from_user(user))))
}

async fn password_change_route(
	Extension(current_user): Extension<CurrentUser>,
	State(UsersState {
		users_repository,
		email_service,
		..
	}): State<UsersState>,
	ValidatedJson(request): ValidatedJson<ChangePasswordRequest>,
) -> ApiResult<()> {
	let password_hash = password::hash(&request.new_password).map_err(|_| UsersApiError::FailedToChangePassword())?;

	let user = users_repository
		.find_user_by_id(&current_user.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(current_user.user_id.to_string()))?;

	if !user.is_verified {
		return Err(UsersApiError::UserNotVerified(current_user.user_id.to_string()))?;
	}

	users_repository
		.change_password(&current_user.user_id, &password_hash)
		.await
		.map_err(|_| UsersApiError::UserNotFound(current_user.user_id.to_string()))?;

	if let Err(error) = email_service.send_password_changed_email(&current_user.email) {
		log::error!("Failed to send password changed email: {error}");
	}

	Ok((StatusCode::OK, ()))
}

#[derive(Debug, Error)]
pub enum UsersApiError {
	#[error("Wrong password")]
	WrongPassword(),

	#[error("User not verified: {0}")]
	UserNotVerified(String),

	#[error("User not found: {0}")]
	UserNotFound(String),

	#[error("Email taken: {0}")]
	EmailTaken(String),

	#[error("Username taken: {0}")]
	UsernameTaken(String),

	#[error("Failed to create user")]
	FailedToCreateUser(),

	#[error("Failed to login user")]
	FailedToLoginUser(),

	#[error("Failed to change password")]
	FailedToChangePassword(),
}

impl IntoResponse for UsersApiError {
	fn into_response(self) -> Response {
		let status_code = match self {
			Self::UserNotVerified(_) | Self::WrongPassword() => StatusCode::BAD_REQUEST,
			Self::UserNotFound(_) => StatusCode::NOT_FOUND,
			Self::EmailTaken(_) | Self::UsernameTaken(_) => StatusCode::CONFLICT,
			Self::FailedToCreateUser() | Self::FailedToLoginUser() | Self::FailedToChangePassword() => {
				StatusCode::INTERNAL_SERVER_ERROR
			}
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
