use std::sync::Arc;

use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::IntoMakeService;
use axum::Json;
use axum::Router;
use thiserror::Error;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::propagate_header::PropagateHeaderLayer;

use crate::database::{self};
use crate::experience::routes::{ExperienceApiError, ExperienceState};
use crate::follows::routes::{FollowApiError, FollowsState};
use crate::settings::SETTINGS;
use crate::users::routes::{UsersApiError, UsersState};
use crate::{experience, follows, users};

pub async fn create_app() -> IntoMakeService<Router> {
	let database = Arc::new(database::Database::init().await.unwrap());

	let users_state = UsersState::new(&database);
	let follows_state = FollowsState::new(&database);
	let experience_state = ExperienceState::new(&database);

	let router = Router::new()
		.merge(users::routes::init().with_state(users_state))
		.merge(follows::routes::init().with_state(follows_state))
		.merge(experience::routes::init().with_state(experience_state))
		.layer(tower_http::trace::TraceLayer::new_for_http())
		.layer(CookieManagerLayer::new())
		.layer(CompressionLayer::new())
		.layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
			"x-request-id",
		)))
		.layer(
			CorsLayer::new()
				.allow_credentials(true)
				.allow_methods([
					Method::GET,
					Method::POST,
					Method::DELETE,
					Method::PUT,
					Method::PATCH,
					Method::OPTIONS,
				])
				.allow_headers([header::CONTENT_TYPE])
				.allow_origin(SETTINGS.website_url.parse::<HeaderValue>().unwrap()),
		);

	router.into_make_service()
}

#[derive(Debug, Error)]
pub enum ApiError {
	#[error("{0}")]
	Users(#[from] UsersApiError),

	#[error("{0}")]
	Follow(#[from] FollowApiError),

	#[error("{0}")]
	Experience(#[from] ExperienceApiError),
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		match self {
			ApiError::Users(error) => error.into_response(),
			ApiError::Follow(error) => error.into_response(),
			ApiError::Experience(error) => error.into_response(),
		}
	}
}

pub type ApiResult<T> = std::result::Result<(StatusCode, T), ApiError>;

pub struct ApiErrorResponse {
	status_code: StatusCode,
	message: String,
}

impl ApiErrorResponse {
	pub fn new(status_code: StatusCode, message: String) -> Self {
		Self { status_code, message }
	}
}

impl IntoResponse for ApiErrorResponse {
	fn into_response(self) -> Response {
		(self.status_code, Json(self.message)).into_response()
	}
}
