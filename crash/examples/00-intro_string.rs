fn main() {
    let hello1 = String::from("Hello, ");
    let hello2 = String::from(", hello!");
    let name = "Alice";
    println!("{}", hello1 + name);
    println!("{}", name.to_string() + &hello2);
}
