use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceDto {
	#[validate(length(min = 1, max = 255, message = "Wrong company name length"))]
	pub company_name: String,
	#[validate(length(min = 1, max = 255, message = "Wrong occupation length"))]
	pub occupation: String,
	#[validate(length(min = 1, max = 255, message = "Wrong location name length"))]
	pub location_name: String,
	#[validate(custom(function = "validate_location_type"))]
	pub location_type: String,
	#[validate(custom(function = "validate_employment_type"))]
	pub employment_type: String,
	pub start_date: DateTime<Utc>,
	pub end_date: Option<DateTime<Utc>>,
	pub is_current: bool,
	#[validate(length(min = 1, message = "Description is required"))]
	pub description: String,
}

fn validate_location_type(location_type: &str) -> Result<(), ValidationError> {
	if ["on-site", "remote", "hybrid"].contains(&location_type) {
		Ok(())
	} else {
		Err(ValidationError::new("Wrong location type"))
	}
}

fn validate_employment_type(employment_type: &str) -> Result<(), ValidationError> {
	if ["full-time", "part-time", "contract", "freelance", "internship"].contains(&employment_type) {
		Ok(())
	} else {
		Err(ValidationError::new("Wrong employment type"))
	}
}
