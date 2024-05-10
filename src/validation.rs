use async_trait::async_trait;
use axum::{
	extract::{rejection::JsonRejection, FromRequest, Json, Request},
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
	T: DeserializeOwned + Validate,
	S: Send + Sync,
	Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
	type Rejection = ServerError;

	async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
		let Json(value) = Json::<T>::from_request(request, state).await?;
		value.validate()?;
		Ok(ValidatedJson(value))
	}
}

#[derive(Debug, Error)]
pub enum ServerError {
	#[error(transparent)]
	ValidationError(#[from] ValidationErrors),

	#[error(transparent)]
	AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
	fn into_response(self) -> Response {
		log::error!("{self}");

		match self {
			ServerError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
			ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, String::from("Server rejection")),
		}
		.into_response()
	}
}
