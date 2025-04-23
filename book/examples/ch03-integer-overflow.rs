// This will panic with integer overflow in debug mode, but produce "0"
// in release mode.
fn overflow() {
    let mut i : u8 = 0;
    for _ in 1..=256  {
        i += 1
    }
    println!("overflow: {i}");
}
// This code uses `wrapping_add` so that it always wraps and never panics
// (even in debug mode).
fn wrapping_overflow() {
    let mut i : u8 = 0;
    for _ in 1..=256  {
        i = i.wrapping_add(1)
    }
    println!("wrapping overflow: {i}");
}
fn main() {
    wrapping_overflow();
    overflow();
}
