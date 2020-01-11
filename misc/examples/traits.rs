// https://blog.rust-lang.org/2015/05/11/traits.html

struct Point {
    x: f64,
    y: f64,
}

// A free-standing function that converts a (borrowed) point to a string.
fn point_to_string(point: &Point) -> String {
    point.to_string()
}

// An "inherent impl" block defines the methods available directly on a type
impl Point {
    // This method is available on any Point, and automatically borrows the
    // Point value.
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("{}", Point { x: 1.9, y: 20.2 }.to_string());
}
