use crate::backend::Backend;

pub struct Donerking;

impl Backend for Donerking {
	fn categories(&self) -> Vec<String> {
		todo!()
	}

	fn menu(&self) -> Vec<String> {
		todo!()
	}

	fn order(&self, dish: &str) {
		todo!()
	}
}
