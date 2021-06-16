pub mod donerking;

use crate::{Menu, Order, error::*};
use core::fmt::Debug;

pub trait Backend {
	type Currency: Debug + rusty_money::FormattableCurrency;
	type Timezone: chrono::TimeZone;

	fn fetch_menu(&self) -> reqwest::Result<Menu>;
	fn place_order(
		&self,
		order: &Order<Self::Currency, Self::Timezone>,
	) -> Result<String, OrderError>;
}
