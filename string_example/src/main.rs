fn main() {
		let mut s = String::new();
		s.push('h');
    println!("Hello, world! - {}", s);
		let data = "initial contents";
		let s2 = data.to_string();
		println!("str to string : {}", s2);
		let mut s3 = String::from("foo");
		s3.push_str("bar");
		println!("mutated string : {}", s3);

		let s4 = String::from("Hello, ");
		let s5 = String::from("world");
		let s6 = s4 + &s5;
		println!("s4 + s5 : {}", s6);

		let s7 = String::from("tick");
		let s8 = String::from("tack");
		let s9 = String::from("tok");
		let result = s7 + &s8 + &s9;
		println!("s + s +s : {}", result);

		let s1 = String::from("hello");
		let h = s1[0]; // error for string can not be accessed with index

		let hello = "Здравствуйте";
		let ss = &hello[0..4];
		println!("rusian char 2 bytes: {}", ss);
}
