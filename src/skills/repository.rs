use std::sync::Arc;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Clone)]
pub struct SkillsRepository {
	pub(crate) database: Arc<Database>,
}

impl SkillsRepository {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			database: Arc::clone(database),
		}
	}

	// TODO: Use inside admin panel
	#[allow(dead_code)]
	pub async fn create(&self, name: &str) -> anyhow::Result<Skill> {
		let skill = sqlx::query_as!(Skill, "INSERT INTO skills (name) VALUES ($1) RETURNING *", name)
			.fetch_one(&*self.database.pool)
			.await
			.context("Failed to follow user")?;

		Ok(skill)
	}

	// TODO: Use inside admin panel
	#[allow(dead_code)]
	pub async fn list(&self) -> anyhow::Result<Vec<Skill>> {
		let skills = sqlx::query_as!(Skill, "SELECT * FROM skills")
			.fetch_all(&*self.database.pool)
			.await
			.context("Failed to fetch skills")?;

		Ok(skills)
	}

	pub async fn get_user_skills(&self, user_id: &i64) -> anyhow::Result<Vec<Skill>> {
		let skills = sqlx::query_as!(
			Skill,
			r#"
				SELECT s.id, s.name
				FROM skills s
				JOIN user_skills us ON s.id = us.skill_id
				WHERE us.user_id = $1;
			"#,
			user_id
		)
		.fetch_all(&*self.database.pool)
		.await
		.context("Failed to get user skills")?;

		Ok(skills)
	}

	pub async fn update_user_skills(&self, user_id: &i64, skills: Vec<Skill>) -> anyhow::Result<()> {
		sqlx::query!("DELETE FROM user_skills WHERE user_id = $1", user_id)
			.execute(&*self.database.pool)
			.await
			.context("Failed to delete user skills")?;

		if skills.is_empty() {
			return Ok(());
		}

		// TODO: Use bulk insert
		for skill in skills {
			sqlx::query!(
				"INSERT INTO user_skills (user_id, skill_id) VALUES ($1, $2)",
				user_id,
				skill.id
			)
			.execute(&*self.database.pool)
			.await
			.context("Failed to insert user skills")?;
		}

		Ok(())
	}
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Skill {
	pub id: i64,
	pub name: String,
}
