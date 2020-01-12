use std::fmt::Debug;

trait P {
    fn p(&self);
}

#[derive(Debug)]
struct A(i32);

impl<T: Debug> P for T {
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
