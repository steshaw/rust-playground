// This will panic with integer overflow in debug mode, but produce "0"
// in release mode.
fn main() {
    let mut i : u8 = 0;
    for _ in 1..=256  {
        i += 1
    }
    println!("{i}");
}
