fn main() {
		// without reference, need to use tuple for returned string
		let s = String::from("hello");
		let (ss, len) = calculate_length_without_reference(s);
		println!("The length of '{}' is '{}'", ss, len);
		// using reference, no need to use tuple 
		let s1 = String::from("hello");
		let len = calculate_length(&s1);
		println!("The length of '{}' is '{}'", s1, len);
}
fn calculate_length_without_reference(s: String) -> (String, usize){
		let length= s.len();
		(s, length)
}
fn calculate_length(s: &String) -> usize {
		s.len()
}
