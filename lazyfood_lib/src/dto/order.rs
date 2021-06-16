use rusty_money::FormattableCurrency;
use chrono::{DateTime, TimeZone};
use crate::{Addition, Address, Dish, Identity};
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct OrderEntry<'a, C: Debug + FormattableCurrency> {
	pub dish: Dish<'a, C>,
	pub count: u8,
	pub additions: Vec<Addition<'a, C>>,
	pub comment: String,
}

#[derive(Debug, Clone)]
pub struct Order<'a, C: Debug + FormattableCurrency, Tz: TimeZone> {
	pub entries: Vec<OrderEntry<'a, C>>,
	pub address: Address,
	pub identity: Identity,
	pub delivery_time: Option<DateTime<Tz>>,
	pub comment: String,
}
