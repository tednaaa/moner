use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	middleware,
	response::{IntoResponse, Response},
	routing::{delete, get, post, put},
	Extension, Json, Router,
};
use thiserror::Error;

use crate::{
	app::{ApiErrorResponse, ApiResult},
	database::Database,
	users::auth::{self, CurrentUser},
	validation::ValidatedJson,
};

use super::{
	dtos::ExperienceDto,
	repository::{Experience, ExperienceRepository},
};

#[derive(Clone)]
pub struct ExperienceState {
	pub experience_repository: ExperienceRepository,
}

impl ExperienceState {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			experience_repository: ExperienceRepository::new(database),
		}
	}
}

pub fn init() -> Router<ExperienceState> {
	Router::new()
		.route("/experience", post(create_experience_route))
		.route("/experience/:experience_id", put(update_experience_route))
		.route("/experience/:experience_id", delete(delete_experience_route))
		.route_layer(middleware::from_fn(auth::middleware))
		.route("/:user_id/experience", get(user_experiences_route))
}

async fn create_experience_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<ExperienceState>,
	ValidatedJson(request): ValidatedJson<ExperienceDto>,
) -> ApiResult<Json<i64>> {
	let experience = state
		.experience_repository
		.create(&current_user.user_id, request)
		.await
		.map_err(|_| ExperienceApiError::Create())?;

	Ok((StatusCode::CREATED, Json(experience.id)))
}

async fn update_experience_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<ExperienceState>,
	Path(experience_id): Path<i64>,
	ValidatedJson(request): ValidatedJson<ExperienceDto>,
) -> ApiResult<()> {
	state
		.experience_repository
		.update(&experience_id, &current_user.user_id, request)
		.await
		.map_err(|_| ExperienceApiError::Update())?;

	Ok((StatusCode::OK, ()))
}

async fn delete_experience_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<ExperienceState>,
	Path(experience_id): Path<i64>,
) -> ApiResult<()> {
	state
		.experience_repository
		.delete(&experience_id, &current_user.user_id)
		.await
		.map_err(|_| ExperienceApiError::Delete())?;

	Ok((StatusCode::OK, ()))
}

async fn user_experiences_route(
	Path(user_id): Path<i64>,
	State(state): State<ExperienceState>,
) -> ApiResult<Json<Vec<Experience>>> {
	let user_experience = state
		.experience_repository
		.get_by_user_id(user_id)
		.await
		.map_err(|_| ExperienceApiError::GetUser())?;

	Ok((StatusCode::CREATED, Json(user_experience)))
}

#[derive(Debug, Error)]
pub enum ExperienceApiError {
	#[error("Failed to get user experience")]
	GetUser(),

	#[error("Failed to create experience")]
	Create(),

	#[error("Failed to update experience")]
	Update(),

	#[error("Failed to delete experience")]
	Delete(),
}

impl IntoResponse for ExperienceApiError {
	fn into_response(self) -> Response {
		let status_code = match self {
			ExperienceApiError::GetUser() => StatusCode::INTERNAL_SERVER_ERROR,
			ExperienceApiError::Create() => StatusCode::INTERNAL_SERVER_ERROR,
			ExperienceApiError::Update() => StatusCode::INTERNAL_SERVER_ERROR,
			ExperienceApiError::Delete() => StatusCode::INTERNAL_SERVER_ERROR,
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
