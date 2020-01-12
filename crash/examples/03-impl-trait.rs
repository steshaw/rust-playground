//use std::fmt::*;

trait P {
    fn p(&self);
}

#[derive(Debug)]
struct A(i32);

impl P for A {
    fn p(&self) {
        println!("{:?}", self);
    }
}

fn foo() -> impl P {
    A(42)
}

fn main() {
    foo().p();
}
