use core::fmt::Debug;
use std::collections::HashMap;
use rusty_money::FormattableCurrency;
use crate::Dish;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
	pub id: String,
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Menu<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub dishes: Vec<Dish<C>>,
	pub categories: HashMap<String, String>,
}
