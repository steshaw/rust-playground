fn message_and_return<'a>(msg: &String, ret: &'a String) -> &'a String {
    println!("msg = {}", msg);
    ret
}
fn main() {
    let a = "a".to_string();
    let b = "b".to_string();
    let r = message_and_return(&a, &b);
    assert_eq!("b", r);
}
