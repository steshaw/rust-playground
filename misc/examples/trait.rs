trait Animal<A> {
    fn is_mammal(self, x: A) -> bool;    // OK             -- mentions a.
    fn number_of_giraffes(self) -> i32;  // NOT OK in Flix -- does not mention a.
}

impl Animal<i32> for i32 {
    fn is_mammal(self, _x: i32) -> bool { true }
    fn number_of_giraffes(self) -> i32 { 1 }
}

fn main() {
    let a = 32;
    println!("Hello, {}!", a.is_mammal(42));
    println!("Hello, {}!", a.number_of_giraffes());
}
