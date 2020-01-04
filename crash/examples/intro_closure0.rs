fn solution1() {
    println!("solution1");
    let hello = "Hello, ";
    let greet = |name| hello.to_string() + name;
    println!("{}", greet("Alice"));
    println!("{}", greet("Bob"));
}

fn solution2() {
    println!("solution2");
    let hello = "Hello, ".to_string();
    let greet = |name| hello.clone() + name;
    println!("{}", greet("Alice"));
    println!("{}", greet("Bob"));
}

fn solution3() {
    println!("solution3");
    let hello = String::from("Hello, ");
    let greet = |name| hello.to_string() + name;
    println!("{}", greet("Alice"));
    println!("{}", greet("Bob"));
}

fn main() {
    solution1();
    solution2();
    solution3();
}
