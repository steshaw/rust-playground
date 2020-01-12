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
        //
        // More string concat alternatives:
        //
        //   https://github.com/hoodie/concatenation_benchmarks-rs
        //
        [
            "(",
            self.x.to_string().as_str(),
            ", ",
            self.y.to_string().as_str(),
            ")",
        ]
        .join("")
    }
}

fn main() {
    let p = Point { x: 1.9, y: 20.2 };
    let s1 = p.to_string(); // implicit borrow.
    let s2 = (&p).to_string(); // explicit borrow.
    let s3 = point_to_string(&p); // required explicit borrow.
    println!("{} {} {}", s1, s2, s3);
}
