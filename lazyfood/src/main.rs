use lazyfood_lib::backend::donerking::Donerking;
use lazyfood_lib::backend::Backend;
use lazyfood_lib::reqwest::Client;

fn main() {
	let donerking = Donerking::new();
	let menu = donerking.fetch_menu().unwrap();
	println!("{:#?}", &menu);
}
