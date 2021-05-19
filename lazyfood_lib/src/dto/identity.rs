use validator::Validate;

#[derive(Debug, Validate)]
pub struct Identity {
	pub first_name: String,
	pub last_name: String,
	#[validate(phone)]
	pub phone_number: String,
	#[validate(email)]
	pub email: String,
}
