use std::collections::HashMap;
fn main() {
		let mut scores = HashMap::new();
		scores.insert(String::from("blue"), 10);
		scores.insert(String::from("red"), 20);
		let team_name = String::from("blue");
		let score = scores.get(&team_name);
    println!("Hello, world!, {}", score.unwrap());
		for (key, value) in &scores {
				println!("key:{}, value: {}", key, value);
		}
		let text = "hello world wonderful world";
		let mut map = HashMap::new();
		for word in text.split_whitespace() {
				let count = map.entry(word).or_insert(0);
				*count += 1;
		}
		println!("{:?}", map);
}
