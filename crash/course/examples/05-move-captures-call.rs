fn main() {
    let name = String::from("Alice");
    // Compile error at println if using `move`: borrow of moved value: `name`.
    let _ = /*move*/ || println!("Hello, {}", name);
    println!("Using name from main: {}", name); // error!
}
