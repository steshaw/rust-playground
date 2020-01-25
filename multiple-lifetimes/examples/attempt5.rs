fn a<'a>(s: &'a mut String, b: &'a str) -> &'a String {
    s.push_str(b);
    s
}

fn main() {
    assert_eq!(a(&mut "hi".to_string(), "there"), "hithere");
}
