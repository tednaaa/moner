use dotenvy::{dotenv, var};
use lazy_static::lazy_static;

lazy_static! {
	pub static ref SETTINGS: Settings = Settings::new();
}

#[derive(Debug, Clone)]
pub struct Settings {
	#[allow(dead_code)] // actually used in logging
	pub rust_log: String,
	pub database_url: String,
	pub website_url: String,

	pub server: Server,
	pub smtp: Smtp,
	pub auth: Auth,
}

impl Settings {
	pub fn new() -> Self {
		dotenv().expect(".env file not found");

		pretty_env_logger::init();

		Self {
			rust_log: get_env("RUST_LOG"),
			database_url: get_env("DATABASE_URL"),
			website_url: get_env("WEBSITE_URL"),

			server: Server {
				port: get_env::<u16>("PORT"),
			},

			smtp: Smtp {
				username: get_env("SMTP_USERNAME"),
				password: get_env("SMTP_PASSWORD"),
				sender_email: get_env("SMTP_SENDER_EMAIL"),
			},

			auth: Auth {
				jwt_secret: get_env("JWT_SECRET"),
			},
		}
	}
}

#[derive(Debug, Clone)]
pub struct Server {
	pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Smtp {
	pub username: String,
	pub password: String,
	pub sender_email: String,
}

#[derive(Debug, Clone)]
pub struct Auth {
	pub jwt_secret: String,
}

fn get_env<T: std::str::FromStr>(key: &str) -> T {
	let value_str = var(key).unwrap_or_else(|_| panic!("{key} must be set"));

	value_str.parse().unwrap_or_else(|_| {
		panic!(
			"Failed to parse env var: {key} must be of type ({})",
			std::any::type_name::<T>()
		)
	})
}
