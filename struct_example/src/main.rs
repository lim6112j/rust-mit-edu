struct User {
		name: String,
		email: String,
		sign_in_count: u64,
		active: bool,
}
// need lifetime for slice , borrowed type
struct UserSlice<'a> {
		name: &'a str,
		email: &'a str,
}
fn main() {
		let user = User {name: String::from("ben"), email: String::from("lim"), sign_in_count: 22, active: true};
		println!("{}", user.name);
		let user2 = UserSlice {name: "ben", email: "@222"};
		println!("{}, {}", user2.name, user2.email);
}

