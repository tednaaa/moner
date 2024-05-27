use anyhow::{Context, Result};
use std::net::SocketAddr;

mod app;
mod database;
mod services;
mod settings;
mod validation;

mod follows;
mod users;

use settings::SETTINGS;

#[tokio::main]
async fn main() -> Result<()> {
	let address = SocketAddr::from(([127, 0, 0, 1], SETTINGS.server.port));

	let app = app::create_app().await;
	let listener = tokio::net::TcpListener::bind(&address)
		.await
		.context("Failed to bind address")?;

	log::info!("✅ Started listening on http://{address}");
	axum::serve(listener, app).await.context("Failed to start server")?;

	Ok(())
}
