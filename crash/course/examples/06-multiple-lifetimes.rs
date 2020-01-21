fn message_and_return<'a>(msg: &String, ret: &'a String) -> &'a String {
    println!("msg = {}", msg);
    ret
}
fn foo(name: &String) -> &String {
    let msg = "C".to_string();
    message_and_return(&msg, &name)
}
fn main() {
    let a = "a".to_string();
    let r = foo(&a);
    assert_eq!("a", r);
}
