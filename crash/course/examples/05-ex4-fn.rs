fn main() {
    call_with_hi(say_message);
    call_with_hi(say_message);
}

fn say_message(msg: &str) {
    println!("{}", msg);
}

fn call_with_hi<F: Fn(&str)>(f: F) {
    f("Hi!");
}
