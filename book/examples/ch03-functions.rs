fn five() -> i32 {
    // Unnecessary `return` — this nicely shows up as a warning in RustRover.
    return 5;
}
fn main() {
    let _ = five();
}
