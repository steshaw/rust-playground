fn simple_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
fn mut_as_assignable() {
    // mut allows the variable to be _reassigned_.
    let mut s = String::from("hi");
    println!("The value of s is: {s}");
    s = String::from("ho");
    println!("The value of s is reassigned to: {s}");
}
fn mut_as_mutable() {
    // mut allows the value referenced to be mutated.
    let mut s = String::from("hi");
    println!("The value of s is: {s}");
    s.clear();
    // Using String::clear makes the &mut more obvious.
    String::clear(&mut s);
    s.push_str("ho");
    // Using String::push_str makes the &mut more obvious.
    String::push_str(&mut s, "ho"); // Now we have "hoho".
    println!("The value of s is changed to: {s}");
}
pub fn main() {
    simple_mut();
    mut_as_assignable();
    mut_as_mutable()
}
