fn needs_mutable(x: &mut u32) {
    *x *= 2;
}

fn needs_immutable(x: &u32) {
    println!("{}", x);
}

fn main() {
    let mut x: u32 = 5;
    let y: &mut u32 = &mut x;
    needs_immutable(y);
    needs_mutable(y);
    needs_immutable(y);

    let mut x: u32 = 20;
    needs_immutable(&x);
    needs_mutable(&mut x);
    needs_immutable(&x);
}
