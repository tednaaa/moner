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
	follows::repository::FollowsRepository,
	services::{email::EmailService, verification::VerificationService},
	validation::ValidatedJson,
};

use super::{
	auth::{self, authorize_jwt, verify_jwt, CurrentUser},
	dtos::{
		ChangePasswordRequest, CreateUserRequest, LoginUserRequest, PublicUserResponse, ResendVerificationRequest,
		ResetPasswordRequest, UserResponse, VerifyPasswordRequest, VerifyUserRequest,
	},
	password,
	repository::UsersRepostory,
};

#[derive(Clone)]
pub struct UsersState {
	pub users_repository: UsersRepostory,
	pub follows_repository: FollowsRepository,
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
			follows_repository: FollowsRepository::new(database),
			email_service: EmailService::new(),
			verification_service: VerificationService::new(),
		}
	}
}

pub fn init() -> Router<UsersState> {
	Router::new()
		.route("/users/me", get(get_me_route))
		.route("/users/delete", delete(delete_user_route))
		.route("/users/password/change", patch(password_change_route))
		.route_layer(middleware::from_fn(auth::middleware))
		.route("/users/register", post(register_user_route))
		.route("/users/verify", patch(verify_user_route))
		.route("/users/resend-verification", post(resend_verification_route))
		.route("/users/password/reset", post(password_reset_route))
		.route("/users/password/verify", post(password_verify_route))
		.route("/users/login", post(login_user_route))
		.route("/users/logout", get(logout_user_route))
		.route("/users/:username", get(get_public_user_route))
		.route("/users/oauth/:provider/callback", get(oauth_callback_route))
		.route("/users/oauth/:provider/login", get(oauth_login_route))
}

async fn register_user_route(
	State(mut state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	state
		.users_repository
		.delete_unverified_user(&request.email)
		.await
		.map_err(|_| UsersApiError::FailedToCreateUser())?;

	let password_hash = password::hash(request.password)
		.await
		.map_err(|_| UsersApiError::FailedToCreateUser())?;

	let created_user = state
		.users_repository
		.create_user(&request.email, &request.username, &password_hash)
		.await
		.map_err(|error| match error.to_string().to_lowercase() {
			string if string.contains("email") => UsersApiError::EmailTaken(request.email.clone()),
			string if string.contains("username") => UsersApiError::UsernameTaken(request.username.clone()),
			_ => UsersApiError::FailedToCreateUser(),
		})?;

	if let Err(error) = state.email_service.send_verification_email(
		&request.email,
		&state.verification_service.generate_code(&created_user.id.to_string()),
	) {
		log::error!("Failed to send verification email: {error}");
	};

	Ok((StatusCode::CREATED, Json(UserResponse::from(created_user))))
}

async fn verify_user_route(
	cookies: Cookies,
	State(mut state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<VerifyUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	let is_verified = state
		.verification_service
		.verify_code(&request.user_id.to_string(), &request.code);

	if !is_verified {
		return Err(UsersApiError::InvalidVerificationCode(request.code.clone()))?;
	}

	let verified_user = state
		.users_repository
		.verify_user(&request.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.user_id.to_string()))?;

	authorize_jwt(cookies, CurrentUser::from(verified_user.clone())).map_err(|_| UsersApiError::FailedToLoginUser())?;

	if let Err(error) = state.email_service.send_welcome_email(&verified_user.email) {
		log::error!("Failed to send welcome email: {error}");
	}

	Ok((StatusCode::OK, Json(UserResponse::from(verified_user))))
}

async fn resend_verification_route(
	State(mut state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<ResendVerificationRequest>,
) -> ApiResult<()> {
	let user = state
		.users_repository
		.find_user_by_id(&request.user_id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.user_id.to_string()))?;

	if !user.is_verified {
		if let Err(error) = state.email_service.send_verification_email(
			&user.email,
			&state.verification_service.generate_code(&user.id.to_string()),
		) {
			log::error!("Failed to send verification email: {error}");
		}
	}

	Ok((StatusCode::ACCEPTED, ()))
}

async fn login_user_route(
	cookies: Cookies,
	State(mut state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<LoginUserRequest>,
) -> ApiResult<Json<UserResponse>> {
	let user = state
		.users_repository
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
		if let Err(error) = state.email_service.send_verification_email(
			&user.email,
			&state.verification_service.generate_code(&user.id.to_string()),
		) {
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

async fn get_public_user_route(
	cookies: Cookies,
	Path(username): Path<String>,
	State(state): State<UsersState>,
) -> ApiResult<Json<PublicUserResponse>> {
	let user = state
		.users_repository
		.find_user_by_username(&username)
		.await
		.map_err(|_| UsersApiError::UserNotFound(username.clone()))?;

	let access_token = cookies
		.get("access_token")
		.map(|cookie| cookie.value().to_string())
		.unwrap_or(String::from(""));

	let mut is_followed = false;

	if let Ok(payload) = verify_jwt(&access_token) {
		is_followed = state
			.follows_repository
			.is_following(&payload.claims.user.user_id, &user.id)
			.await
			.map_err(|_| UsersApiError::UserNotFound(username.clone()))?;
	}

	let followers_count = state
		.follows_repository
		.get_followers_count(&user.id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(username.clone()))?;

	let following_count = state
		.follows_repository
		.get_following_count(&user.id)
		.await
		.map_err(|_| UsersApiError::UserNotFound(username.clone()))?;

	Ok((
		StatusCode::OK,
		Json(PublicUserResponse::from_user(
			user,
			is_followed,
			followers_count,
			following_count,
		)),
	))
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

async fn password_reset_route(
	State(mut state): State<UsersState>,
	ValidatedJson(request): ValidatedJson<ResetPasswordRequest>,
) -> ApiResult<()> {
	let user = state
		.users_repository
		.find_user_by_email(&request.email)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.email.to_string()))?;

	if let Err(error) = state.email_service.send_password_reset_email(
		&user.email,
		&state.verification_service.generate_code(&user.id.to_string()),
	) {
		log::error!("Failed to send password reset email: {error}");
	}

	Ok((StatusCode::ACCEPTED, ()))
}

async fn password_verify_route(
	cookies: Cookies,
	State(UsersState {
		users_repository,
		mut verification_service,
		..
	}): State<UsersState>,
	ValidatedJson(request): ValidatedJson<VerifyPasswordRequest>,
) -> ApiResult<Json<UserResponse>> {
	let user = users_repository
		.find_user_by_email(&request.email)
		.await
		.map_err(|_| UsersApiError::UserNotFound(request.email.to_string()))?;

	if !user.is_verified {
		return Err(UsersApiError::UserNotVerified(request.email.to_string()))?;
	}

	let is_verified = verification_service.verify_code(&user.id.to_string(), &request.code);
	if !is_verified {
		return Err(UsersApiError::InvalidVerificationCode(request.code))?;
	}

	authorize_jwt(cookies, CurrentUser::from(user.clone())).map_err(|_| UsersApiError::FailedToLoginUser())?;

	Ok((StatusCode::OK, Json(UserResponse::from(user))))
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
	let password_hash = password::hash(request.new_password.clone())
		.await
		.map_err(|_| UsersApiError::FailedToChangePassword())?;

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

	#[error("Failed to change password")]
	FailedToChangePassword(),
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
			UsersApiError::FailedToChangePassword() => StatusCode::INTERNAL_SERVER_ERROR,
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
