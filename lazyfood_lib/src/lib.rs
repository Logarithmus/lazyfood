pub mod backend;
mod dto;
mod error;

pub use error::Error;
pub use dto::{dish::*, order::*, address::*, identity::*};
pub use backend::Backend;

use reqwest::{IntoUrl, Client};
use select::{
	document::Document,
	predicate::{Predicate, Attr, Name, Element},
};
use rusty_money::{Money, FormattableCurrency, iso};
use std::collections::HashMap;

#[tokio::main]
async fn get_document<U: IntoUrl>(client: &Client, url: U) -> Result<Document, reqwest::Error> {
	let response = client.get(url).send().await?;
	let text = response.text().await?;
	Ok(Document::from(text.as_str()))
}

fn get_categories(doc: &Document) -> HashMap<String, String> {
	let cat_predicate = Name("div").and(Attr("id", "filters"));
	doc.find(cat_predicate)
		.into_selection()
		.children()
		.filter(Name("button"))
		.iter()
		.map(|el| {
			(el.attr("data-filter").unwrap()[1..].to_string(), el.first_child().unwrap().text())
		})
		.filter(|cat| !cat.0.is_empty())
		.collect()
}

fn get_dishes(doc: &Document) -> Vec<Dish<impl FormattableCurrency>> {
	let dishes_predicate = Name("div").and(Attr("data-isotope-options", ()));
	let dishes_selection = doc.find(dishes_predicate).into_selection();
	dishes_selection
		.children()
		.filter(Element)
		.iter()
		.map(|el| {
			let order_button =
				el.find(Name("button").and(Attr("title", "Заказать"))).next().unwrap();
			let parent_id = el
				.attr("class")
				.unwrap()
				.split_ascii_whitespace()
				.find(|word| word.starts_with("parent_id"))
				.unwrap();
			let dish_info = el.find(Attr("class", "imageDataInfoRow")).next().unwrap();
			let mass_text = dish_info.find(Attr("class", "left")).next().unwrap().text();
			let price_node = dish_info.find(Attr("class", "right")).next().unwrap();
			let price_text = price_node.text();
			Dish {
				id: order_button.attr("data-id").unwrap().to_string(),
				category: parent_id.to_string(),
				name: order_button.attr("data-title").unwrap().to_string(),
				ingredients: "unknown".to_string(),
				variants: vec![DishVariant {
					mass: mass_text,
					price: Money::from_minor(str::parse(&price_text).unwrap(), iso::BYN),
				}]
			}
		})
		.collect()
}

pub fn execute() -> Result<(), reqwest::Error> {
	let client = reqwest::Client::new();
	let document = get_document(&client, "https://donerking.by")?;
	let _categories = get_categories(&document);
	let dishes = get_dishes(&document);
	//dbg!(&categories, &dishes);
	dishes
		.iter()
		.filter(|dish| dish.category == "parent_id_26")
		.for_each(|dish| println!("{}", dish));
	Ok(())
}
