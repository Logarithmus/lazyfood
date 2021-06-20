use core::fmt::Display;
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct Identity {
	pub first_name: String,
	pub last_name: String,
	#[validate(phone)]
	pub phone_number: String,
	#[validate(email)]
	pub email: String,
}
