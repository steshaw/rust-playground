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
fn simple_const() {
    const X: u32 = 5;
    println!("The value of the X constant is: {X}");

    // We cannot shadow `x` here with `const` because constants have different
    // rules to variables. We can still shadow with a `let`.
    // This is interesting but not idiomatic as constants are usually uppercase.
    // Ack: this actually causes a "refutable pattern in local binding" compile
    // error that seemingly isn't pickup up by RustRover/rust-analyser.
    // Have to rename constant to `X` to make this work.
    // Missing patterns are not covered because `X` is interpreted as a constant
    // pattern, not a new variable.
    let x = 6;
    println!("The value of x variable is: {x}");
}
pub fn main() {
    simple_mut();
    mut_as_assignable();
    mut_as_mutable();
    simple_const();
}
