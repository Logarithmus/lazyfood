use rusty_money::FormattableCurrency;
use chrono::{DateTime, TimeZone};
use crate::{Addition, Address, Dish, Identity};

pub struct OrderEntry<'a, C: FormattableCurrency> {
	pub dish: Dish<'a, C>,
	pub count: u8,
	pub additions: Vec<Addition<'a, C>>,
	pub pita_type: String,
	pub comment: String,
}

pub struct Order<'a, C: FormattableCurrency, Tz: TimeZone> {
	pub dishes: Vec<OrderEntry<'a, C>>,
	pub address: Address,
	pub identity: Identity,
	pub delivery_time: DateTime<Tz>,
	pub comment: String,
}
