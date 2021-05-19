use derive_more::Display;
use rusty_money::{FormattableCurrency, Money, iso};

#[derive(Debug)]
pub struct Dish<'a, C: FormattableCurrency> {
	pub id: String,
	pub category: String,
	pub name: String,
	pub ingredients: String,
	pub variants: Vec<DishVariant<'a, C>>,
}

#[derive(Debug)]
pub struct DishVariant<'a, C: FormattableCurrency> {
	pub mass: String,
	pub price: Money<'a, C>,
}

pub struct AdditionVariant<'a, C: FormattableCurrency> {
	pub mass: String,
	pub price: Money<'a, C>,
}

pub struct Addition<'a, C: FormattableCurrency> {
	pub id: String,
	pub name: String,
	pub variants: Vec<AdditionVariant<'a, C>>,
}
