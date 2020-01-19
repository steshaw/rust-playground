fn eg1() {
    let mut x = 5;

    let _y = &x;

    // Works with NLL.
    let _z = &mut x;
}

fn eg2() {
    let mut x = 5;
    let y = &x;
    //let z = &mut x; // [rustc E0502] [E] cannot borrow `x` as mutable because it is also borrowed as immutable.

    println!("y: {}", y);
}

fn main() {
    eg1();
    eg2();
}
