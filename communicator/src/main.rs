extern crate communicator;
pub mod a {
		pub mod series {
				pub mod of {
						pub fn nested_modules() {}
				}
		}
}
use a::series::of;
use a::series::of::nested_modules;

enum TrafficLight {
		Red,
		Blue,
		Green,
}
use TrafficLight::{Red, Blue};
fn main() {
		communicator::client::connect();
		a::series::of::nested_modules();
		of::nested_modules();
		nested_modules();
		let red = Red;
		let blue = Blue;
}
