pub mod backend;

use reqwest::{IntoUrl, Client};
use select::{document::Document, predicate::{Predicate, Attr, Name, Element}};
use core::{fmt::{Debug, Display}, iter::Iterator};
use std::collections::HashMap;


pub struct Money(u32);

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0 / 100, self.0 % 100)
    }
}

impl Debug for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Debug)]
pub struct Dish<'a> {
	pub id: String,
	pub category_id: &'a str,
	pub name: String,
	pub mass: String,
	pub price: Money,
}

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
		.map(|el| (el.attr("data-filter").unwrap()[1..].to_string(), el.first_child().unwrap().text()))
		.filter(|cat| !cat.0.is_empty())
		.collect()
}

fn get_dishes(doc: &Document) -> Vec<Dish> {
	let dishes_predicate = Name("div").and(Attr("data-isotope-options", ()));
	let dishes_selection = doc.find(dishes_predicate).into_selection();
	dishes_selection
		.children()
		.filter(Element)
		.iter()
		.map(|el| {
			let order_button = el.find(Name("button").and(Attr("title", "Заказать"))).next().unwrap();
			let parent_id = el.attr("class").unwrap().split_ascii_whitespace()
				.find(|word| word.starts_with("parent_id"))
				.unwrap();
			let dish_info = el.find(Attr("class", "imageDataInfoRow")).next().unwrap();
			let mass_text = dish_info.find(Attr("class", "left")).next().unwrap().text();
			let price_node = dish_info.find(Attr("class", "right")).next().unwrap();
			let price_text = price_node.text();
			Dish {
				id: order_button.attr("data-id").unwrap().to_string(),
				category_id: parent_id,
				name: order_button.attr("data-title").unwrap().to_string(),
				mass: mass_text,
				price: Money(str::parse(&price_text).unwrap()),
			}
		})
		.collect()
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let document = get_document(&client, "https://donerking.by")?;
	let categories = get_categories(&document);
	let dishes = get_dishes(&document);
	//dbg!(&categories, &dishes);
	dishes.iter()
		.filter(|dish| dish.category_id == "parent_id_26")
		.for_each(|dish| println!("{:#?}", dish));
    Ok(())
}
