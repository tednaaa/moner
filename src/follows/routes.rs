use std::sync::Arc;

use axum::{
	extract::State,
	http::StatusCode,
	middleware,
	response::{IntoResponse, Response},
	routing::post,
	Extension, Router,
};
use thiserror::Error;

use super::{
	dtos::{FollowUserRequest, UnfollowUserRequest},
	repository::FollowsRepository,
};
use crate::{
	app::{ApiErrorResponse, ApiResult},
	database::Database,
	users::auth::{self, CurrentUser},
	validation::ValidatedJson,
};

#[derive(Clone)]
pub struct FollowsState {
	pub follow_repository: FollowsRepository,
}

impl FollowsState {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			follow_repository: FollowsRepository::new(database),
		}
	}
}

pub fn init() -> Router<FollowsState> {
	Router::new()
		.route("/follow", post(follow_route))
		.route("/unfollow", post(unfollow_route))
		.route_layer(middleware::from_fn(auth::middleware))
}

async fn follow_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<FollowsState>,
	ValidatedJson(request): ValidatedJson<FollowUserRequest>,
) -> ApiResult<()> {
	if current_user.user_id == request.followed_id {
		return Err(FollowApiError::FailedToFollowUser())?;
	}

	state
		.follow_repository
		.follow_user(&current_user.user_id, &request.followed_id)
		.await
		.map_err(|_| FollowApiError::FailedToFollowUser())?;

	Ok((StatusCode::CREATED, ()))
}

async fn unfollow_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<FollowsState>,
	ValidatedJson(request): ValidatedJson<UnfollowUserRequest>,
) -> ApiResult<()> {
	if current_user.user_id == request.unfollowed_id {
		return Err(FollowApiError::FailedToUnfollowUser())?;
	}

	state
		.follow_repository
		.unfollow_user(&current_user.user_id, &request.unfollowed_id)
		.await
		.map_err(|_| FollowApiError::FailedToUnfollowUser())?;

	Ok((StatusCode::OK, ()))
}

#[derive(Debug, Error)]
pub enum FollowApiError {
	#[error("Failed to follow user")]
	FailedToFollowUser(),

	#[error("Failed to unfollow user")]
	FailedToUnfollowUser(),
}

impl IntoResponse for FollowApiError {
	fn into_response(self) -> Response {
		let status_code = match self {
			FollowApiError::FailedToFollowUser() => StatusCode::BAD_REQUEST,
			FollowApiError::FailedToUnfollowUser() => StatusCode::BAD_REQUEST,
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
