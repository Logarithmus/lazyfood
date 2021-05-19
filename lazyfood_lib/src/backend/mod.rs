pub mod donerking;

use rusty_money::FormattableCurrency;
use chrono::TimeZone;
use crate::{Dish, Order};

pub trait Backend<C: FormattableCurrency, Tz: TimeZone> {
	fn fetch_menu(&self) -> Vec<Dish<C>>;
	fn place_order(&self, order: &Order<C, Tz>);
}
