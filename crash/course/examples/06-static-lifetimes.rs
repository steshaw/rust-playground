fn msg() -> &'static str {
    "Hi"
}

fn main() {
    let msg: &str = msg();
    println!("{}", msg);
}
