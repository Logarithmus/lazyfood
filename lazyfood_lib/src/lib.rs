pub mod backend;
mod dto;
mod error;

pub use error::*;
pub use dto::{menu::*, dish::*, order::*, address::*, identity::*};
pub use backend::Backend;
pub use reqwest;

use reqwest::{IntoUrl, Client};
use select::{
	document::Document,
	predicate::{Predicate, Attr, Name, Element},
};
use rusty_money::{Money, FormattableCurrency, iso};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
