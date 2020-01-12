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

    let child_e = Command::new("/home/steshaw/.nix-profile/bin/echo")
        .arg("Hello,")
        .arg("world!")
        .current_dir(".")
        .stdout(Stdio::piped())
        .spawn();
    let r = child_e
        .and_then(|child| match child.wait_with_output() {
            Ok(out) => {
                println!("out = {:?}", out);
                println!("out.stdout = {:?}", out.stdout);
                assert_eq!(out.stdout, b"Hello, world!\n");
                Ok(out)
            }
            Err(err) => Err(err),
        })
        .or_else(|e| {
            println!("An error occurred: {}", e);
            Err(e)
        });
    println!("r = {:?}", r); // Use r :-)
}
