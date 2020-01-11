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
        // Horrors when not using format!.
        // I bet format! finds the nicest way to do it.
        let mut r = "(".to_string();
        r.push_str(self.x.to_string().as_str());
        r.push_str(", ");
        r.push_str(self.y.to_string().as_str());
        r.push_str(")");
        r
    }
}

fn main() {
    println!("{}", Point { x: 1.9, y: 20.2 }.to_string());
}
