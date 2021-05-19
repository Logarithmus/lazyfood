use rusty_money::FormattableCurrency;
use chrono::TimeZone;
use crate::{Backend, Dish, Order};

pub struct Donerking;

impl<C: FormattableCurrency, Tz: TimeZone> Backend<C, Tz> for Donerking {
	fn fetch_menu(&self) -> Vec<Dish<C>> {
		todo!()
	}

	fn place_order(&self, _order: &Order<C, Tz>) {
		todo!()
	}
}
