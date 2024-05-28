use std::sync::Arc;

use anyhow::{anyhow, Context};
use sqlx::postgres::PgQueryResult;

use crate::database::Database;

#[derive(Clone)]
pub struct FollowsRepository {
	pub(crate) database: Arc<Database>,
}

impl FollowsRepository {
	pub fn new(database: &Arc<Database>) -> Self {
		Self {
			database: Arc::clone(database),
		}
	}

	pub async fn follow_user(&self, follower_id: &i64, followed_id: &i64) -> anyhow::Result<PgQueryResult> {
		let query_result = sqlx::query_as!(
			Follows,
			"INSERT INTO follows (follower_id, followed_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
			follower_id,
			followed_id
		)
		.execute(&*self.database.pool)
		.await
		.context("Failed to follow user")?;

		Ok(query_result)
	}

	pub async fn unfollow_user(&self, follower_id: &i64, followed_id: &i64) -> anyhow::Result<PgQueryResult> {
		let query_result = sqlx::query_as!(
			Follows,
			"DELETE FROM follows WHERE follower_id = $1 AND followed_id = $2",
			follower_id,
			followed_id
		)
		.execute(&*self.database.pool)
		.await
		.context("Failed to unfollow user")?;

		Ok(query_result)
	}

	pub async fn is_following(&self, follower_id: &i64, followed_id: &i64) -> anyhow::Result<bool> {
		let query_result = sqlx::query!(
			r#"
			SELECT EXISTS (
				SELECT 1
				FROM follows
				WHERE follower_id = $1 AND followed_id = $2
			) AS is_followed
			"#,
			follower_id,
			followed_id
		)
		.fetch_one(&*self.database.pool)
		.await
		.map_err(|_| anyhow!("Failed to check if user is following"))?;

		Ok(query_result.is_followed.unwrap_or(false))
	}

	// TODO: add pagination
	#[allow(dead_code)]
	pub async fn get_followers(&self, followed_id: &i64) -> anyhow::Result<Vec<i64>> {
		let query_result = sqlx::query!("SELECT follower_id FROM follows WHERE followed_id = $1", followed_id)
			.fetch_all(&*self.database.pool)
			.await
			.context("Failed to get followers")?;

		Ok(query_result.iter().map(|row| row.follower_id).collect())
	}

	// TODO: add pagination
	#[allow(dead_code)]
	pub async fn get_following(&self, follower_id: &i64) -> anyhow::Result<Vec<i64>> {
		let query_result = sqlx::query!("SELECT followed_id FROM follows WHERE follower_id = $1", follower_id)
			.fetch_all(&*self.database.pool)
			.await
			.context("Failed to get following")?;

		Ok(query_result.iter().map(|row| row.followed_id).collect())
	}

	pub async fn get_followers_count(&self, followed_id: &i64) -> anyhow::Result<i64> {
		let query_result = sqlx::query!("SELECT COUNT(*) FROM follows WHERE followed_id = $1", followed_id)
			.fetch_one(&*self.database.pool)
			.await
			.context("Failed to get followers count")?;

		Ok(query_result.count.unwrap_or(0))
	}

	pub async fn get_following_count(&self, follower_id: &i64) -> anyhow::Result<i64> {
		let query_result = sqlx::query!("SELECT COUNT(*) FROM follows WHERE follower_id = $1", follower_id)
			.fetch_one(&*self.database.pool)
			.await
			.context("Failed to get following count")?;

		Ok(query_result.count.unwrap_or(0))
	}
}

#[derive(sqlx::FromRow)]
pub struct Follows {
	pub follower_id: i64,
	pub followed_id: i64,
}
