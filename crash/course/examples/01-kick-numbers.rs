fn main() {
    let i = 0x2a;
    print(i);
    print(i);

    let s = String::from("Fred");
    print_string(s.clone());
    print_string(s);

    let s = String::from("Wilma");
    print_string_ref(&s);
    print_string_ref(&s);
}

fn print(i: u8) {
    println!("i = {}", i)
}

fn print_string(s: String) {
    println!("s = {}", s)
}

fn print_string_ref(s: &String) {
    println!("s = {}", s)
}
