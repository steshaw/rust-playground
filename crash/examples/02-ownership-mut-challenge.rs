#[derive(Debug)]
struct I32(i32);

impl Copy for I32 {}

// NOTE: This Clone is different to Copy.
impl Clone for I32 {
    fn clone(self: &Self) -> I32 {
        I32(1) // NOTE: Not using self here!
    }
}

fn main() {
    let x = I32(42);

    foo(x);
    foo(x); // Allowed by Copy.
    foo(x.clone()); // Allowed by Copy.

    let mut y = I32(2);

    bar(&y);
    bar(&y);

    let z = &mut y;
    bar(&&&&z /*  &y*/);
    baz(z /*&mut y*/);
    baz(z);
}

// move
fn foo(i: I32) {
    println!("foo: i = {:?}", i);
}

// read only reference
fn bar(_i: &I32) {}

// mutable reference
fn baz(_i: &mut I32) {}
