use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32,u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.contains_key(&arg) {
            true => {
                arg
            },
            false => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    for v in c.value.iter() {
        println!("{}: {}", v.0, v.1);
    }
    assert_eq!(v2,2);
    assert_eq!(c.value.len(),2);
}
#[test]
fn closure_test() {
    let x = vec![1,2,3];
    let equal_to_x = move |z| z==x;
    //println!("cant use x in here : {:?}", x);
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}
