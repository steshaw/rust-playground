fn floop<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        &"Hello"
    }
}
fn main() {
    let a = floop("this", "that");
    assert_eq!(a, "Hello");
}
