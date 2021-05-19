use derive_more::Display;
use rusty_money::{FormattableCurrency, Money, iso};

#[derive(Debug, Display)]
#[display(
	fmt = "Dish {{\n\
	\tname:        {},\n\
	\tingredients: {},\n\
	\tmass:        {},\n\
	\tprice:       {}\n}}",
	name,
	ingredients,
	mass,
	price
)]
pub struct Dish<'a, C: FormattableCurrency> {
	pub id: String,
	pub category_id: String,
	pub name: String,
	pub ingredients: String,
	pub mass: String,
	pub price: Money<'a, C>,
}

impl<'a> Default for Dish<'a, iso::Currency> {
	fn default() -> Self {
		Self {
			id: "unknown".into(),
			category_id: "unknown".into(),
			name: "unknown".into(),
			ingredients: "unknown".into(),
			mass: "unknown".into(),
			price: Money::from_minor(0, iso::USD),
		}
	}
}

pub struct AdditionVariant<'a, C: FormattableCurrency> {
	pub mass: String,
	pub cost: Money<'a, C>,
}

pub struct Addition<'a, C: FormattableCurrency> {
	pub id: String,
	pub name: String,
	pub available_variants: Vec<AdditionVariant<'a, C>>,
}
