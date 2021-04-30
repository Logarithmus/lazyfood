pub mod donerking;

pub trait Backend {
	fn categories(&self) -> Vec<String>;
	fn menu(&self) -> Vec<String>;
	fn order(&self, dish: &str);
}
