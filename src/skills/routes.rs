use std::sync::Arc;

use axum::{
	debug_handler,
	extract::{Path, State},
	http::StatusCode,
	middleware,
	response::{IntoResponse, Response},
	routing::{get, post},
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
	dtos::UpdateUserSkillsDto,
	repository::{Skill, SkillsRepository},
};

#[derive(Clone)]
pub struct SkillsState {
	pub skills_repository: SkillsRepository,
}

impl SkillsState {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			skills_repository: SkillsRepository::new(database),
		}
	}
}

pub fn init() -> Router<SkillsState> {
	Router::new()
		.route("/skills", post(update_user_skills_route))
		.route("/skills/suggestions", get(get_suggestions))
		.route_layer(middleware::from_fn(auth::middleware))
		.route("/skills/:user_id", get(get_user_skills_route))
}

async fn get_suggestions(State(state): State<SkillsState>) -> ApiResult<Json<Vec<Skill>>> {
	let skills = state
		.skills_repository
		.list()
		.await
		.map_err(|_| SkillsApiError::GetSuggestions())?;

	Ok((StatusCode::CREATED, Json(skills)))
}

async fn get_user_skills_route(
	Path(user_id): Path<i64>,
	State(state): State<SkillsState>,
) -> ApiResult<Json<Vec<Skill>>> {
	let user_skills = state
		.skills_repository
		.get_user_skills(&user_id)
		.await
		.map_err(|_| SkillsApiError::GetUserSkills())?;

	Ok((StatusCode::CREATED, Json(user_skills)))
}

#[debug_handler]
async fn update_user_skills_route(
	Extension(current_user): Extension<CurrentUser>,
	State(state): State<SkillsState>,
	ValidatedJson(request): ValidatedJson<UpdateUserSkillsDto>,
) -> ApiResult<()> {
	state
		.skills_repository
		.update_user_skills(&current_user.user_id, request.skills)
		.await
		.map_err(|_| SkillsApiError::UpdateUserSkills())?;

	Ok((StatusCode::OK, ()))
}

#[derive(Debug, Error)]
pub enum SkillsApiError {
	#[error("Failed to get skills suggestions")]
	GetSuggestions(),

	#[error("Failed to get user skills")]
	GetUserSkills(),

	#[error("Failed to update user skills")]
	UpdateUserSkills(),
}

impl IntoResponse for SkillsApiError {
	fn into_response(self) -> Response {
		let status_code = match self {
			SkillsApiError::GetUserSkills() => StatusCode::INTERNAL_SERVER_ERROR,
			SkillsApiError::UpdateUserSkills() => StatusCode::BAD_REQUEST,
			SkillsApiError::GetSuggestions() => StatusCode::INTERNAL_SERVER_ERROR,
		};

		log::error!("{self:?}");
		ApiErrorResponse::new(status_code, self.to_string()).into_response()
	}
}
