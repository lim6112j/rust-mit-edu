//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-02-deref.html
use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// define own smart pointer
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
use List::{Cons, Nil};
fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // ref ddref
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);

    let a = MyBox::new(x);
    assert_eq!(5, *a);
}
