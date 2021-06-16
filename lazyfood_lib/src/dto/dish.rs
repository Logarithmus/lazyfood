use core::{
	fmt::{Debug, Formatter},
	convert::TryFrom,
};
use rusty_money::{FormattableCurrency, Money};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Dish<'a, C: Debug + FormattableCurrency> {
	pub id: String,
	pub category: String,
	pub name: String,
	pub ingredients: Vec<String>,
	pub nutrition_facts: Option<NutritionFacts>,
	pub variants: Vec<DishVariant<'a, C>>,
	pub additions: Vec<Addition<'a, C>>,
}

#[derive(Clone)]
pub struct DishVariant<'a, C: Debug + FormattableCurrency> {
	pub name: String,
	pub quantity: Quantity,
	pub price: Money<'a, C>,
}

impl<'a, C: Debug + FormattableCurrency> Debug for DishVariant<'_, C> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"DishVariant {{\n    \
			quantity: {:?}\n    \
			price: {}\n\
			}}",
			self.quantity, self.price
		)
	}
}

#[derive(Debug, Clone)]
pub enum Quantity {
	Grams(f32),
	Milliliters(f32),
	Centimeters(f32),
	Pieces(u32),
}

#[derive(thiserror::Error, Debug)]
pub enum QuantityError {
	#[error("Quantity number not found in \"{0}\"")]
	NumberNotFound(String),
}

impl TryFrom<&str> for Quantity {
	type Error = QuantityError;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref QUANTITY_REGEX: Regex = Regex::new("(0?)[\\.,]?(\\d*)\\s*(\\w*)").unwrap();
		}
		let cap = QUANTITY_REGEX.captures(s).unwrap();
		dbg!(&cap[1], &cap[2]);
		let num_low: u32 = cap[2].parse().unwrap();
		let num = if &cap[1] == "0" {
			num_low as f32 / (10_u32.pow(cap[2].len() as u32) as f32)
		} else {
			num_low as f32
		};
		let q = &cap[3];
		Ok(if q.starts_with("г") {
			Self::Grams(num)
		} else if q.starts_with("мл") {
			Self::Milliliters(num)
		} else if q.starts_with("л") {
			Self::Milliliters(1000.0 * num)
		} else if q.starts_with("см") {
			Self::Centimeters(num)
		} else {
			Self::Pieces(num_low)
		})
	}
}

#[derive(Debug, Clone)]
pub struct NutritionFacts {
	pub proteins: f32,
	pub fats: f32,
	pub carbohydrates: f32,
	pub kilocalories: f32,
}

#[derive(Debug, Clone)]
pub struct Addition<'a, C: Debug + FormattableCurrency> {
	pub id: String,
	pub name: String,
	pub variants: Vec<AdditionVariant<'a, C>>,
}

#[derive(Clone)]
pub struct AdditionVariant<'a, C: Debug + FormattableCurrency> {
	pub quantity: Quantity,
	pub price: Money<'a, C>,
}

impl<'a, C: Debug + FormattableCurrency> Debug for AdditionVariant<'_, C> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"AdditionVariant {{\n    \
			quantity: {:?}\n    \
			price: {}\n\
			}}",
			self.quantity, self.price
		)
	}
}
