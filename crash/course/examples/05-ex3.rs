fn main() {
    let msg: &str = "Hi!";
    fn say_hi(s: &str) {
        println!("{}", s);
    }
    say_hi(msg);
    say_hi(msg);
}

