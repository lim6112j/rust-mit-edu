enum IpAddrKind {
		V4,
		V6,
}
struct IpAddr {
		kind: IpAddrKind,
		address: String,
}
enum IpAddr2 {
		V4(String),
		V6(String),
}
// enum is simpler than struct below
enum Message {
		Quit,
		Move {x: i32, y: i32}, // anonymous struct
		Write(String),
		ChangeColor(i32,i32,i32),
}
// enum Message to struct , verbose
struct QuitMessage;
struct MoveMessage {
		x: i32,
		y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32,i32); //tuple struct

impl Message {
		fn call(&self) {
				println!("called Message::call method")
		}
}
#[derive(Debug)]
enum UsState {
		Alabama,
		NewYork,
}
enum Coin {
		Penny,
		Nickel,
		Dime,
		Quater(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
		match coin {
				Coin::Penny => 1,
				Coin::Nickel => 5,
				Coin::Dime => 10,
				Coin::Quater(state) => {
						println!("state is {:?}", state);
						25
				},
		}
}

fn route(ip_type: IpAddrKind){
		
}

fn main() {
		let four = IpAddrKind::V4;
		let six = IpAddrKind::V6;
		let home = IpAddr {kind: four, address: String::from("http://home")};
		let loopback = IpAddr {kind: six, address: String::from("http://loop")};
		let home2 = IpAddr2::V4(String::from("home"));
		let loopback2 = IpAddr2::V6(String::from("::1"));

		let message = Message::Write(String::from("hello"));
		message.call();
		let a_coin = Coin::Quater(UsState::Alabama);
		let value = value_in_cents(a_coin);
		println!("quater is {}", value);
}
