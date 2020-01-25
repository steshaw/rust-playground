fn second<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}
fn first<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    (a, second(b, a))
}
fn zero<'a>(a: &'a str, _unused: &str) -> (&'a str, &'a str) {
    first(a, "b")
}
fn main() {
    assert_eq!(zero("a", "unused"), ("a", "b"));
}
