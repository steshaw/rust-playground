use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let a : &str = &arg;
        match a {
            "hi" => println!("Hello there!"),
            "bye" => println!("OK, goodbye!"),
            _ => println!("Sorry, I don't know what {} means", arg),
        }
    }
}
