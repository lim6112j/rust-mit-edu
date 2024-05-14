fn plus_one(x: Option<i32>) -> Option<i32> {
		match x {
				None => None,
				Some(value) => Some(value + 1),
		}
}
fn main() {
		let x = Some(5);
		let y = plus_one(x);
		println!("option plus one value : {}", y.unwrap());
		let z = plus_one(None);
}
