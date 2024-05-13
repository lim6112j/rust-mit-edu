fn main() {
		let mut s1 = String::from("hello");
		let _s2 = s1;
		// intentional error, memory borrowed, shallow copy like but called as 'move'
		// we can call this process 's1 moved to s2'
		s1.push_str(", world!");
		println!("{}",s1);
		// Integer has known size at compile time and stored entirely on stack, works as if deep copy.
		let x = 4;
		let y = x;
		println!("x={}, y={}", x, y);
}
