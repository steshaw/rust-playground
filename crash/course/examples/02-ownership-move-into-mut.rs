#[derive(Clone, Copy, Debug)]
struct R {
    c: char,
    i: i32,
}

fn main() {
    let x = R { c: 'x', i: 1 };

    // NOTE: x moving into mut variable.
    // NOTE: Would it nice of this was explicit with `foo(mut x)`?
    foo(x);

    // Can access x here if R is Copy.
    println!("main: x = {:?}", x);
}

fn foo(mut x: R) {
    x.i = 2;
    println!("foo x = {:?}", x);
}
