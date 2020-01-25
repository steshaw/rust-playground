fn message_and_return<'a>(msg: &str, ret: &'a str) -> &'a str {
    println!("Printing the message: {}", msg);
    ret
}

fn foo(name: &str) -> &str {
    let msg = String::from("This is the message");
    message_and_return(&msg, &name)
}

fn main() {
    let name = String::from("Alice");
    let ret = foo(&name);
    println!("Return value: {}", ret);
}
