use rusty_money::{Money, iso};
use reqwest::{
	Client,
	header::{self, HeaderMap, HeaderValue},
	IntoUrl, Url,
};
use select::{document::Document, predicate::*, node::Node};
use std::collections::HashMap;
use crate::{OrderError, backend::Backend, dto::{address::*, dish::*, identity::*, menu::*, order::*}};
use regex::Regex;
use core::convert::TryFrom;
use serde::Serialize;

lazy_static! {
	static ref SITE_URL: Url = Url::parse("https://donerking.by").unwrap();
	static ref USER_AGENT: HeaderValue = HeaderValue::from_static(
		"Mozilla/5.0 (X11; Linux x86_64) \
		AppleWebKit/537.36 (KHTML, like Gecko) \
		Chrome/90.0.4430.85 Safari/537.36"
	);
	static ref FE_REQUEST_URL: Url = SITE_URL.join("/ajax/fe_request.php").unwrap();
}

const CURRENCY: &iso::Currency = iso::BYN;
const TIMEZONE: chrono_tz::Tz = chrono_tz::Europe::Minsk;

pub struct Donerking {
	client: Client,
}

impl Donerking {
	pub fn new() -> Self {
		Donerking {
			client: Client::builder()
				.user_agent(&*USER_AGENT)
				.cookie_store(true)
				.build()
				.unwrap(),
		}
	}

	#[tokio::main]
	async fn get_document(&self, url: impl IntoUrl) -> reqwest::Result<Document> {
		let response = self.client.get(url).send().await?;
		let text = response.text().await?;
		Ok(Document::from(text.as_str()))
	}

	fn extract_categories(doc: &Document) -> HashMap<String, String> {
		doc.find(Name("div").and(Attr("id", "filters")))
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

	fn extract_ingredients(dish_doc: &Document) -> Vec<String> {
		let ingredients_node = dish_doc
			.find(Class("catalog__single").descendant(Name("p").or(Name("ul"))))
			.next()
			.and_then(|node| node.parent());
		ingredients_node.map_or_else(
			|| vec![],
			|node| {
				node.find(Name("ul"))
					.into_selection()
					.children()
					.filter(Name("li"))
					.iter()
					.map(|el| el.text())
					.collect()
			},
		)
	}

	fn extract_nutrition_facts(dish_doc: &Document) -> Option<NutritionFacts> {
		let nutrition_facts_node = dish_doc
			.find(Class("catalog__single").descendant(Name("p").or(Name("ul"))))
			.next()
			.and_then(|node| node.parent());
		let nutrition_facts_text = nutrition_facts_node.and_then(|node| {
			node.find(Name("p")).map(|node| node.text()).find(|text| text.starts_with("Пищевая"))
		});

		nutrition_facts_text.and_then(|text| {
			lazy_static! {
				static ref NUTRITION_FACTS_REGEX: Regex = Regex::new(
					"белки.*?(\\d+,?\\d*).*?жиры.*?(\\d+,?\\d*).*?углеводы.*?(\\d+,?\\d*)\
					.*?Энергетическая ценность.*?(\\d+,?\\d*).*?"
				)
				.unwrap();
			}
			let cap = NUTRITION_FACTS_REGEX.captures(&text).unwrap();
			let facts: Option<Vec<f32>> =
				(1..=4).map(|i| cap[i].replace(",", ".").parse().ok()).collect();
			facts.map(|facts| NutritionFacts {
				proteins: facts[0],
				fats: facts[1],
				carbohydrates: facts[2],
				kilocalories: facts[3],
			})
		})
	}

	#[tokio::main]
	async fn query_additions(
		&self,
		action: &str,
		key: &str,
		id: &str,
		lang: &str,
	) -> reqwest::Result<String> {
		let request = self
			.client
			.post(FE_REQUEST_URL.clone())
			.header("x-requested-with", HeaderValue::from_static("XMLHttpRequest"))
			.form(&[("action", action), ("key", key), ("getdishdetails", id), ("lang", lang)])
			.build()
			.unwrap();
		self.client.execute(request).await?.text().await
	}

	fn extract_additions_html(&self, dish_doc: &Document) -> reqwest::Result<String> {
		use serde_json::Value;
		let add_to_cart_form = dish_doc.find(Attr("id", "catalog__info")).next();
		Ok(add_to_cart_form.map_or_else(
			|| String::new(),
			|node| {
				let action = node.attr("data-action").unwrap();
				let key = node.attr("data-key").unwrap();
				let id = node.attr("data-itemid").unwrap();
				let lang = node.attr("data-lang").unwrap();
				let json: Value =
					serde_json::from_str(&self.query_additions(action, key, id, lang).unwrap())
						.unwrap();
				match &json["orderDishData"] {
					Value::String(s) => s.clone(),
					_ => String::new(),
				}
			},
		))
	}

	fn extract_additions(additions_doc: &Document) -> Vec<Addition<iso::Currency>> {
		additions_doc
			.find(Name("tr").and(Class("souce_row")))
			.into_selection()
			.filter(Element)
			.iter()
			.map(|node| Addition {
				id: "unknown".to_owned(),
				name: node.find(Name("td").and(Class("souce_row__title"))).next().unwrap().text(),
				variants: {
					let mut variants: Vec<_> = node
						.find(Name("select").and(Class("additionsize")))
						.into_selection()
						.children()
						.filter(Name("option"))
						.iter()
						.map(|node| AdditionVariant {
							quantity: Quantity::try_from(node.text().as_ref()).unwrap(),
							price: Money::from_str(
								node.attr("data-price").unwrap().replace(".", ",").as_str(),
								CURRENCY,
							)
							.unwrap(),
						})
						.collect();
					variants.sort_unstable_by(|x, y| x.price.cmp(&y.price));
					variants
				},
			})
			.collect()
	}

	fn extract_pita_types<'a>(additions_doc: &Document) -> Vec<DishFeature<iso::Currency>> {
		additions_doc
			.find(Name("label").and(Class("radio-container")))
			.into_selection()
			.iter()
			.map(|node| node.find(Text).next())
			.filter_map(|opt| opt)
			.map(|node| DishFeature {
				name: node.text(),
				price: Money::from_minor(0, CURRENCY),
			})
			.collect()
	}

	fn extract_features<'a>(additions_doc: &Document, dish_type: DishType) -> DishFeatures<iso::Currency> {
		match dish_type {
			DishType::Kebab => DishFeatures::Kebab {
			    pita_types: Self::extract_pita_types(additions_doc),
			},
			DishType::Pizza => DishFeatures::Pizza {
			    base_types: vec![],
			    crust_types: vec![],
			},
			DishType::Other => DishFeatures::Other,
		}
	}

	fn get_menu(&self) -> reqwest::Result<Menu<<Self as Backend>::Currency>> {
		let index_doc = self.get_document(SITE_URL.clone())?;
		let categories = Self::extract_categories(&index_doc);
		let dishes_predicate = Name("div").and(Attr("data-isotope-options", ()));
		let dishes_selection = index_doc.find(dishes_predicate).into_selection();
		let dishes = dishes_selection
			.children()
			.filter(Element)
			.iter()
			.map(|el| {
				let order_button =
					el.find(Name("button").and(Attr("title", "Заказать"))).next().unwrap();
				let category_id = el
					.attr("class")
					.unwrap()
					.split_ascii_whitespace()
					.find(|word| word.starts_with("parent_id"))
					.unwrap();
				let info_row = el.find(Attr("class", "imageDataInfoRow")).next().unwrap();
				let quantity_text = info_row.find(Attr("class", "left")).next().unwrap().text();
				let price_text = info_row.find(Attr("class", "right")).next().unwrap().text();

				let btn_row = el.find(Attr("class", "imageDataBtnRow")).next().unwrap();
				let dish_link = btn_row
					.find(Attr("class", "left").child(Name("a")))
					.next()
					.unwrap()
					.attr("href")
					.unwrap();
				let dish_doc = self.get_document(SITE_URL.join(dish_link).unwrap()).unwrap();

				let category_name = categories.get(category_id).cloned().unwrap_or("Unknown".to_owned());
				let additions_html = self.extract_additions_html(&dish_doc).unwrap();
				let additions_doc = Document::from(additions_html.as_str());
				let additions = Self::extract_additions(&additions_doc);
				let features = Self::extract_features(&additions_doc, DishType::from(category_name.as_str()));
				let dish = Dish {
					id: order_button.attr("data-id").unwrap().to_string(),
					category: category_name.clone(),
					name: order_button.attr("data-title").unwrap().to_string(),
					ingredients: Self::extract_ingredients(&dish_doc),
					nutrition_facts: Self::extract_nutrition_facts(&dish_doc),
					variants: vec![DishVariant {
						name: "unknown".to_owned(),
						quantity: Quantity::try_from(quantity_text.as_str()).unwrap(),
						price: Money::from_minor(str::parse(&price_text).unwrap(), CURRENCY),
					}],
					features,
					additions
				};
				dbg!(&dish);
				dish
			})
			.collect();
		let menu = Menu { dishes, categories };
		println!("{}", serde_json::to_string_pretty(&menu).unwrap());
		Ok(menu)
	}
}

impl Backend for Donerking {
	type Currency = iso::Currency;
	type Timezone = chrono::FixedOffset;

	fn fetch_menu(&self) -> reqwest::Result<Menu<Self::Currency>> {
		self.get_menu()
	}

	fn place_order(
		&self,
		_order: &Order<Self::Currency>,
	) -> Result<String, OrderError> {
		todo!()
	}
}
