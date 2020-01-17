use std::fmt;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// NOTE: A Debug implementation is illegal along with Copy.
/*
impl Drop for Point {
    fn drop(&mut self) {
        unimplemented!()
    }
}
*/

impl std::fmt::Display for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // Piggyback of Debug for tuple.
        write!(fmt, "{:?}", (self.x, self.y))
    }
}

fn main() {
    let p = Point { x: 0, y: 0 };
    let p1 = p;
    println!("{}", p);
    println!("{}", p);
    println!("{}", p1);
    println!("{}", p1);
}
