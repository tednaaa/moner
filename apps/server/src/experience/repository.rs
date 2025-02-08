use std::sync::Arc;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::dtos::ExperienceDto;

#[derive(Clone)]
pub struct ExperienceRepository {
	pub(crate) database: Arc<Database>,
}

impl ExperienceRepository {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			database: Arc::clone(database),
		}
	}

	pub async fn create(&self, user_id: &i64, experience_dto: ExperienceDto) -> anyhow::Result<Experience> {
		let experience = sqlx::query_as!(
			Experience,
			r#"
				INSERT INTO experience (user_id, company_name, occupation, location_name, location_type, employment_type, start_date, end_date, is_current, description)
				VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
				RETURNING *
			"#,
			user_id,
			experience_dto.company_name,
			experience_dto.occupation,
			experience_dto.location_name,
			experience_dto.location_type,
			experience_dto.employment_type,
			experience_dto.start_date,
			experience_dto.end_date,
			experience_dto.is_current,
			experience_dto.description
		).fetch_one(&*self.database.pool).await.map_err(|error| anyhow!(error).context("Failed to create experience"))?;

		Ok(experience)
	}

	pub async fn update(&self, id: &i64, user_id: &i64, experience_dto: ExperienceDto) -> anyhow::Result<Experience> {
		let experience = sqlx::query_as!(
			Experience,
			r#"
				UPDATE experience
				SET company_name = $3, occupation = $4, location_name = $5, location_type = $6, employment_type = $7, start_date = $8, end_date = $9, is_current = $10, description = $11
				WHERE id = $1 AND user_id = $2
				RETURNING *
			"#,
			id, user_id,
			experience_dto.company_name,
			experience_dto.occupation,
			experience_dto.location_name,
			experience_dto.location_type,
			experience_dto.employment_type,
			experience_dto.start_date,
			experience_dto.end_date,
			experience_dto.is_current,
			experience_dto.description,

		).fetch_one(&*self.database.pool).await.map_err(|error| anyhow!(error).context("Failed to update experience"))?;

		Ok(experience)
	}

	pub async fn delete(&self, id: &i64, user_id: &i64) -> anyhow::Result<()> {
		sqlx::query!("DELETE FROM experience WHERE id = $1 AND user_id = $2", id, user_id)
			.execute(&*self.database.pool)
			.await
			.map_err(|error| anyhow!(error).context("Failed to delete experience"))?;

		Ok(())
	}

	pub async fn get_by_user_id(&self, user_id: i64) -> anyhow::Result<Vec<Experience>> {
		let experience = sqlx::query_as!(
			Experience,
			"SELECT * FROM experience WHERE user_id = $1 ORDER BY start_date DESC",
			user_id
		)
		.fetch_all(&*self.database.pool)
		.await
		.map_err(|error| anyhow!(error).context("Failed to get experience by user id"))?;

		Ok(experience)
	}
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
	user_id: i64,
	pub id: i64,
	pub company_name: String,
	pub occupation: String,
	pub location_name: String,
	pub location_type: String,
	pub employment_type: String,
	pub start_date: DateTime<Utc>,
	pub end_date: Option<DateTime<Utc>>,
	pub is_current: bool,
	pub description: String,
}
