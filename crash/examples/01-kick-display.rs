use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::result::Result;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        formatter.buf(self.name);
        formatter.buf(self.age);
        Ok(())
    }
}

fn main() {
    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    println!("Display Alice = {}", alice);
    println!("Debug Alice = {:?}", alice);
}
