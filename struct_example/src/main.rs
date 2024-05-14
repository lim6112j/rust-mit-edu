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
fn area(width: usize, height: usize) -> usize {
		width * height
}
#[derive(Debug)]
struct Rectangle {
		width: u32,
		height: u32,
}
impl Rectangle {
		fn area(&self) -> u32 {
				self.width * self.height
		}
		fn can_hold(&self, other: &Rectangle) -> bool {
				self.width > other.width && self.height > other.height
		}
}
fn main() {
		let user = User {name: String::from("ben"), email: String::from("lim"), sign_in_count: 22, active: true};
		println!("{}", user.name);
		let user2 = UserSlice {name: "ben", email: "@222"};
		println!("{}, {}", user2.name, user2.email);
		let height1 = 22;
		let width2 = 11;
		let ar = area(width2, height1);
		println!("area is {:?}", ar);
		let rect1 = Rectangle {width: 30, height: 20};
		println!("{:#?} area is {}", rect1, rect1.area());
		let rect2 = Rectangle {width: 11, height: 12};
		let rect3 = Rectangle {width: 40, height: 11};
		println!("rect1 can hold {:?} : {}", rect2, rect1.can_hold(&rect2));
		println!("rect1 can hold {:?} : {}", rect2, rect1.can_hold(&rect3));
				
}

