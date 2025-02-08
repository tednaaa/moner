use std::sync::Arc;

use anyhow::{anyhow, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::settings::SETTINGS;

pub struct Database {
	pub pool: Arc<Pool<Postgres>>,
}

impl Database {
	pub async fn init() -> Result<Self> {
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(&SETTINGS.database_url)
			.await
			.map_err(|error| anyhow!("Failed to connect to the database: {}", error))?;

		log::info!("✅ PostgreSQL Connection status: OK");

		sqlx::migrate!()
			.run(&pool)
			.await
			.map_err(|error| anyhow!("Failed to apply migrations: {}", error))?;

		log::info!("✅ PostgreSQL Migrations status: OK");

		Ok(Self { pool: Arc::new(pool) })
	}
}
