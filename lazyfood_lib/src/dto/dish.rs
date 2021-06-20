use core::{
	fmt::{Debug, Formatter},
	convert::TryFrom,
};
use rusty_money::{FormattableCurrency, Money};
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Dish<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub id: String,
	pub category: String,
	pub name: String,
	pub ingredients: Vec<String>,
	pub features: DishFeatures<C>,
	pub nutrition_facts: Option<NutritionFacts>,
	pub variants: Vec<DishVariant<C>>,
	pub additions: Vec<Addition<C>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum DishFeatures<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	Kebab { pita_types: Vec<DishFeature<C>> },
	Pizza { base_types: Vec<DishFeature<C>>, crust_types: Vec<DishFeature<C>> },
	Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DishType {
	Kebab,
	Pizza,
	Other
}

impl From<&str> for DishType {
    fn from(category_name: &str) -> Self {
		let category_lowercase = category_name.to_lowercase();
    	if ["кебаб", "шаурма"].iter().any(|s| category_lowercase.contains(s)) {
    		Self::Kebab
    	} else if category_lowercase.contains("пицца") {
			Self::Pizza
		} else {
			Self::Other
		}
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct DishVariant<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub name: String,
	pub quantity: Quantity,
	pub price: Money<'static, C>,
}

impl<C: 'static + Debug + FormattableCurrency> Debug for DishVariant<C>
where &'static C: Deserialize<'static> {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Quantity {
	Grams(f32),
	Milliliters(f32),
	Centimeters(f32),
	Pieces(u32),
}

#[derive(Serialize, Deserialize, thiserror::Error, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NutritionFacts {
	pub proteins: f32,
	pub fats: f32,
	pub carbohydrates: f32,
	pub kilocalories: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Addition<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub id: String,
	pub name: String,
	pub variants: Vec<AdditionVariant<C>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct AdditionVariant<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub quantity: Quantity,
	pub price: Money<'static, C>,
}

impl<C: Debug + FormattableCurrency> Debug for AdditionVariant<C>
where &'static C: Deserialize<'static> {
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct DishFeature<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub name: String,
	pub price: Money<'static, C>,
}

impl<C: Debug + FormattableCurrency> Debug for DishFeature<C>
where &'static C: Deserialize<'static> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"DishFeature {{\n    \
			name: {}\n    \
			price: {}\n\
			}}",
			self.name, self.price
		)
	}
}
