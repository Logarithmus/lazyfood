use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
	pub locality: String,
	pub street: String,
	pub house: String,
	pub flat: String,
}
