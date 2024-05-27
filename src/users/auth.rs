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

#[allow(dead_code)]
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
#[allow(dead_code)]
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
#[allow(dead_code)]
const GOOGLE_REDIRECT_URL: &str = "http://localhost:8080/auth/google/callback";

#[allow(dead_code)]
const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
#[allow(dead_code)]
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[allow(dead_code)]
const DISCORD_AUTH_URL: &str = "https://discord.com/api/oauth2/authorize";
#[allow(dead_code)]
const DISCORD_TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

//	 let token_result = client
// 		.exchange_code(AuthorizationCode::new("some authorization code".to_string()))
// 		.set_pkce_verifier(pkce_verifier)
// 		.request(http_client)?;

#[allow(dead_code)]
pub fn new_google_client() -> Result<BasicClient> {
	let google_client = BasicClient::new(
		ClientId::new(SETTINGS.auth.google_client_id.clone()),
		Some(ClientSecret::new(SETTINGS.auth.google_client_secret.clone())),
		AuthUrl::new(String::from(GOOGLE_AUTH_URL))?,
		Some(TokenUrl::new(String::from(GOOGLE_TOKEN_URL))?),
	)
	.set_redirect_uri(RedirectUrl::new("https://redirect".to_string())?);

	Ok(google_client)
}

#[allow(dead_code)]
pub fn new_github_client() -> Result<BasicClient> {
	let github_client = BasicClient::new(
		ClientId::new(SETTINGS.auth.github_client_id.clone()),
		Some(ClientSecret::new(SETTINGS.auth.github_client_secret.clone())),
		AuthUrl::new(String::from(GITHUB_AUTH_URL))?,
		Some(TokenUrl::new(String::from(GITHUB_TOKEN_URL))?),
	)
	.set_redirect_uri(RedirectUrl::new("https://redirect".to_string())?);

	Ok(github_client)
}

#[allow(dead_code)]
pub fn new_discord_client() -> Result<BasicClient> {
	let discord_client = BasicClient::new(
		ClientId::new(SETTINGS.auth.discord_client_id.clone()),
		Some(ClientSecret::new(SETTINGS.auth.discord_client_secret.clone())),
		AuthUrl::new(String::from(DISCORD_AUTH_URL))?,
		Some(TokenUrl::new(String::from(DISCORD_TOKEN_URL))?),
	)
	.set_redirect_uri(RedirectUrl::new("https://redirect".to_string())?);

	#[allow(unused_variables)]
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	#[allow(unused_variables)]
	let (auth_url, csrf_token) = discord_client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("identify".to_string()))
		.set_pkce_challenge(pkce_challenge)
		.url();

	println!("Browse to: {:#?}", auth_url);

	Ok(discord_client)
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
