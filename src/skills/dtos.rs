use serde::Deserialize;
use validator::Validate;

use super::repository::Skill;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserSkillsDto {
	pub skills: Vec<Skill>,
}
