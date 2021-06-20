pub mod donerking;

use crate::{Menu, Order, error::*};
use core::fmt::Debug;
use serde::Deserialize;

pub trait Backend
where 
	&'static Self::Currency: Deserialize<'static>,
	<Self as Backend>::Currency: 'static
{
	type Currency: Debug + rusty_money::FormattableCurrency;
	type Timezone: chrono::TimeZone;

	fn fetch_menu(&self) -> reqwest::Result<Menu<Self::Currency>>;
	fn place_order(
		&self,
		order: &Order<Self::Currency>,
	) -> Result<String, OrderError>;
}
