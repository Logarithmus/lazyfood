use core::fmt::Debug;
use std::collections::HashMap;
use rusty_money::FormattableCurrency;
use crate::Dish;

#[derive(Debug, Clone)]
pub struct Category {
	pub id: String,
	pub name: String,
}

#[derive(Debug, Clone)]
pub struct Menu<'a> {
	pub dishes: Vec<Dish<'a, rusty_money::iso::Currency>>,
	pub categories: HashMap<String, String>,
}
