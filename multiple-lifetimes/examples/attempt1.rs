fn second<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}
fn first<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    (a, second(b, a))
}
fn main() {
    assert_eq!(first("a", "b"), ("a", "b"));
}
