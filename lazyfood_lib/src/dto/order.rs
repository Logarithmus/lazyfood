use rusty_money::FormattableCurrency;
use chrono::{DateTime, FixedOffset, TimeZone};
use crate::{Addition, Address, Dish, Identity};
use core::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct OrderEntry<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub dish: Dish<C>,
	pub count: u8,
	pub additions: Vec<Addition<C>>,
	pub comment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Order<C: 'static + Debug + FormattableCurrency>
where &'static C: Deserialize<'static> {
	pub entries: Vec<OrderEntry<C>>,
	pub address: Address,
	pub identity: Identity,
	pub delivery_time: Option<DateTime<FixedOffset>>,
	pub comment: String,
}
