#[derive(Debug)]
enum SpreadSheetCell {
		Int(i32),
		Float(f64),
		Text(String),
}
fn main() {
		let row = vec![
				SpreadSheetCell::Int(3),
				SpreadSheetCell::Float(22.2),
				SpreadSheetCell::Text(String::from("hello")),
		];
    println!("Hello, world! {:?}", row);
}
