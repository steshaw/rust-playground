// https://blog.rust-lang.org/2015/05/11/traits.html

use std::process::*;

struct Point {
    x: f64,
    y: f64,
}

// A free-standing function that converts a (borrowed) point to a string.
fn point_to_string(point: &Point) -> String {
    point.to_s()
}

// An "inherent impl" block defines the methods available directly on a type
impl Point {
    // This method is available on any Point, and automatically borrows the
    // Point value.
    fn to_s(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.9, y: 20.2 };
    let s1 = p.to_s(); // implicit borrow.
    let s2 = (&p).to_s(); // explicit borrow.
    let s3 = point_to_string(&p); // required explicit borrow.
    println!("{} {} {}", s1, s2, s3);

    let r = Command::new("scripts/hello")
        .arg("Hello,")
        .arg("world!")
        .current_dir(".")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match r {
        Ok(out) => {
            println!("out = {:?}", out);
            println!("out.stdout = {:?}", String::from_utf8(out.stdout.clone()));
            println!("out.stderr = {:?}", String::from_utf8(out.stderr.clone()));
            assert_eq!(out.stdout, b"Hello, world!\n");
            assert_eq!(out.stderr, b"Boo\n");
        }
        Err(err) => println!("I cannae spawn captain: {}", err),
    }
}
