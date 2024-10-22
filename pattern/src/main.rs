fn main() {
    println!("Hello, world!");
		let favorite_color: Option<&str> = None;
		let is_tuesday = false;
		let age: Result<u8, _> = "34".parse();
		if let Some(color) = favorite_color {
				println!("Using your favorite color, {} as the background", color);
		} else if is_tuesday {
				println!("tuesday is green day");
		} else if let Ok(age) = age {
				if age > 30 {
						println!("using purple as the background color");
				} else {
						println!("Using orange as the background color");
				}
		} else {
				println!("Using blue as the background color");
		}
		let mut stack = Vec::new();
		stack.push(1);
		stack.push(2);
		stack.push(3);
		while let Some(top) = stack.pop() {
				println!("{}",top);
		}

		let v = vec!['a','b','c'];
		for (index, value) in v.iter().enumerate() {
				println!("{} is at index {}", value, index);
		}
		let x = 1;
		match x {
				1 | 2 => println!("one or two"),
				3 => println!("three"),
				_ => println!("anything"),
		}
		match x {
				1 ..= 5 => println!("one through five"),
				_ => println!("something else"),
		}
		let y = 'c';
		match y {
				'a' ..= 'j' => println!("early ascii letter"),
				'k' ..= 'z' => println!("later ascii letter"),
				_ => println!("something else"),
		}
		// destructuring reference
		struct Point {
				x: i32,
				y: i32,
		}
		let points = vec![
				Point {x: 0, y: 0},
				Point {x: 1, y: 5},
				Point {x: 10, y: -3},
		];
		let sum_of_squares: i32 = points
				.iter()
				.map(|&Point {x, y}| x * x + y * y)
				.sum();
		println!("sum of squares : {}", sum_of_squares);
		// create ref in pattern
		let robot_name = Some(String::from("Bors"));
		match robot_name {
				Some(ref name) => println!("Found a name : {}", name),
				None => (),
		}
		println!("robot name: {:?}", robot_name);
		// match guards
		let num = Some(4);
		match num {
				Some(x) if x < 5 => println!("less than five: {}", x),
				Some(x) => println!("{}", x),
				None => (),
		}
}
