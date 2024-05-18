use std::{sync::Arc, thread, time::Duration};

use axum::{
	extract::{Path, State},
	http::StatusCode,
	middleware,
	response::{IntoResponse, Response},
	routing::{delete, get, patch, post},
	Extension, Json, Router,
};
use thiserror::Error;
use tower_cookies::{Cookie, Cookies};

use crate::{
	app::{ApiErrorResponse, ApiResult},
	database::Database,
	services::{email::EmailService, verification::VerificationService},
	validation::ValidatedJson,
};

use super::{
	auth::{self, authorize_jwt, CurrentUser},
	dtos::{ActivateUserRequest, CreateUserRequest, LoginUserRequest, PublicUserResponse, UserResponse},
	password,
	repository::UsersRepostory,
};

#[derive(Clone)]
pub struct UsersState {
	pub users_repository: UsersRepostory,
	pub email_service: EmailService,
	pub verification_service: VerificationService,
}

impl UsersState {
	pub fn new(database: &Arc<Database>) -> Self {
		let verification_service = VerificationService::new();
		let mut verification_service_copy = verification_service.clone();

		thread::spawn(move || loop {
			verification_service_copy.cleanup_expired_codes();
			thread::sleep(Duration::from_secs(60));
		});

		Self {
			users_repository: UsersRepostory::new(database),
			email_service: EmailService::new(),
			verification_service: VerificationService::new(),
		}
	}
}

pub fn init() -> Router<UsersState> {
	Router::new()
		.route("/users/me", get(get_me_route))
		.route("/users/delete", delete(delete_user_route))
		.route("/users/password/reset", post(reset_password_route))
		.route("/users/password/change", patch(change_password_route))
		.route_layer(middleware::from_fn(auth::middleware))
		.route("/users/register", post(register_user_route))
		.route("/users/activate", patch(activate_user_route))
		.route("/users/login", post(login_user_route))
		.route("/users/logout", get(logout_user_route))
		.route("/users/:username", get(get_user_route))
		.route("/users/oauth/:provider/callback", get(oauth_callback_route))
		.route("/users/oauth/:provider/login", get(oauth_login_route))
}

async fn register_user_route(
	State(UsersState {
		users_repository,
		email_service,
		mut verification_service,
	}): State<UsersState>,
	ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	users_repository
		.delete_unverified_user(&request.email)
		.await
		.map_err(|_| UsersApiError::FailedToCreateUser())?;

	let password_hash = password::hash(request.password)
		.await
		.map_err(|_| UsersApiError::FailedToCreateUser())?;

	let created_user = users_repository
		.create_user(&request.email, &request.username, &password_hash)
		.await
		.map_err(|error| match error.to_string().to_lowercase() {
			string if string.contains("email") => UsersApiError::EmailTaken(request.email.clone()),
			string if string.contains("username") => UsersApiError::UsernameTaken(request.username.clone()),
			_ => UsersApiError::FailedToCreateUser(),
		})?;

	if let Err(error) = email_service.send_verification_email(
		&request.email,
		&verification_service.generate_code(&created_user.id.to_string()),
	) {
		log::error!("Failed to send verification email: {error}");
	};

	Ok((StatusCode::CREATED, Json(UserResponse::from(created_user))))
}

async fn activate_user_route(
	State(UsersState {
		users_repository,
		email_service,
		mut verification_service,
	}): State<UsersState>,
	ValidatedJson(request): ValidatedJson<ActivateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	let is_verified = verification_service.verify_code(&request.user_id.to_string(), &request.code);

	if !is_verified {
		return Err(UsersApiError::InvalidVerificationCode(request.code.clone()))?;
	}

	let activated_user = users_repository
		.activate_user(&request.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.user_id.to_string()))?;

	if let Err(error) = email_service.send_welcome_email(&activated_user.email) {
		log::error!("Failed to send welcome email: {error}");
	}

	Ok((StatusCode::OK, Json(UserResponse::from(activated_user))))
}

async fn login_user_route(
	cookies: Cookies,
	State(UsersState {
		users_repository,
		email_service,
		mut verification_service,
	}): State<UsersState>,
	ValidatedJson(request): ValidatedJson<LoginUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	let user = users_repository
		.find_user_by_login(&request.login)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.login.clone()))?;

	let is_valid_password = password::verify(request.password, user.password.clone())
		.await
		.map_err(|_| UsersApiError::WrongPassword())?;

	if !is_valid_password {
		return Err(UsersApiError::WrongPassword())?;
	}

	if !user.is_verified {
		if let Err(error) = email_service
			.send_verification_email(&user.email, &verification_service.generate_code(&user.id.to_string()))
		{
			log::error!("Failed to send verification email: {error}");
		}

		return Err(UsersApiError::UserNotVerified(user.id.to_string()))?;
	}

	authorize_jwt(cookies, CurrentUser::from(user.clone())).map_err(|_| UsersApiError::FailedToLoginUser())?;
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

async fn get_user_route(
	Path(username): Path<String>,
	State(UsersState { users_repository, .. }): State<UsersState>,
) -> ApiResult<Json<PublicUserResponse>> {
	let user = users_repository
		.find_user_by_username(&username)
		.await
		.map_err(|_| UsersApiError::UserNotFound(username))?;

	Ok((StatusCode::OK, Json(PublicUserResponse::from(user))))
}

async fn delete_user_route(
	Extension(current_user): Extension<CurrentUser>,
	State(UsersState { users_repository, .. }): State<UsersState>,
) -> ApiResult<()> {
	users_repository
		.delete_user_by_id(&current_user.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(current_user.user_id.to_string()))?;

	Ok((StatusCode::OK, ()))
}

async fn reset_password_route(
	Extension(current_user): Extension<CurrentUser>,
	State(UsersState {
		users_repository,
		email_service,
		mut verification_service,
	}): State<UsersState>,
) -> ApiResult<()> {
	let user = users_repository
		.find_user_by_id(&current_user.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(current_user.user_id.to_string()))?;

	if let Err(error) =
		email_service.send_password_reset_email(&user.email, &verification_service.generate_code(&user.id.to_string()))
	{
		log::error!("Failed to send verification email: {error}");
	}

	Ok((StatusCode::ACCEPTED, ()))
}

async fn change_password_route() {}
async fn oauth_callback_route() {}
async fn oauth_login_route() {}

#[derive(Debug, Error)]
pub enum UsersApiError {
	#[error("Invalid verification code: {0}")]
	InvalidVerificationCode(String),

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
}

impl IntoResponse for UsersApiError {
	fn into_response(self) -> Response {
		let status_code = match self {
			UsersApiError::InvalidVerificationCode(_) => StatusCode::BAD_REQUEST,
			UsersApiError::UserNotVerified(_) => StatusCode::BAD_REQUEST,
			UsersApiError::WrongPassword() => StatusCode::BAD_REQUEST,
			UsersApiError::UserNotFound(_) => StatusCode::NOT_FOUND,
			UsersApiError::EmailTaken(_) => StatusCode::CONFLICT,
			UsersApiError::UsernameTaken(_) => StatusCode::CONFLICT,
			UsersApiError::FailedToCreateUser() => StatusCode::INTERNAL_SERVER_ERROR,
			UsersApiError::FailedToLoginUser() => StatusCode::INTERNAL_SERVER_ERROR,
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
