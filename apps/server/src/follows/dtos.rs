use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct FollowUserRequest {
	pub followed_id: i64,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct UnfollowUserRequest {
	pub unfollowed_id: i64,
}
