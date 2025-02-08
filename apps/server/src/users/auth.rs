use anyhow::{anyhow, Error, Result};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use oauth2::{
	basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::settings::SETTINGS;

use super::repository::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUser {
	pub user_id: i64,
	pub email: String,
	pub username: String,
}

impl CurrentUser {
	pub fn new(user_id: &i64, email: &str, username: &str) -> Self {
		Self {
			user_id: *user_id,
			email: email.to_string(),
			username: username.to_string(),
		}
	}
}

impl From<User> for CurrentUser {
	fn from(user: User) -> Self {
		Self {
			user_id: user.id,
			email: user.email,
			username: user.username,
		}
	}
}

pub async fn middleware(cookies: Cookies, mut request: Request, next: Next) -> Result<Response, StatusCode> {
	let access_token = cookies
		.get("access_token")
		.map(|cookie| cookie.value().to_string())
		.unwrap_or_else(|| String::from(""));

	if let Ok(payload) = verify_jwt(&access_token) {
		request.extensions_mut().insert(payload.claims.user);
		Ok(next.run(request).await)
	} else {
		Err(StatusCode::UNAUTHORIZED)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
	pub exp: i64,
	pub iat: i64,
	pub user: CurrentUser,
}

pub fn authorize_jwt(cookies: Cookies, user: CurrentUser) -> Result<()> {
	let access_token = create_jwt(&user.user_id, &user.email, &user.username)
		.map_err(|error| anyhow!(error).context("Failed to create JWT"))?;

	let access_token_cookie = Cookie::build(("access_token", access_token))
		.path("/")
		.http_only(true)
		.build();

	println!("{:#?}", access_token_cookie);

	cookies.add(access_token_cookie);
	Ok(())
}

fn create_jwt(user_id: &i64, email: &str, username: &str) -> Result<String, Error> {
	let now = Utc::now().timestamp();
	let claims = Claims {
		exp: now + Duration::hours(12).num_seconds(),
		iat: now,
		user: CurrentUser::new(user_id, email, username),
	};

	encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(SETTINGS.auth.jwt_secret.as_bytes()),
	)
	.map_err(|error| anyhow!(error).context("Failed to encode JWT"))
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
	decode(
		token,
		&DecodingKey::from_secret(SETTINGS.auth.jwt_secret.as_bytes()),
		&Validation::default(),
	)
	.map_err(|error| anyhow!(error).context("Failed to decode JWT"))
}
