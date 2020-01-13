#[derive(Debug)]
struct I32(i32);

impl Drop for I32 {
    fn drop(self: &mut Self) {
        println!("Dropping a I32: {:?}", self);
    }
}

// A "bare" impl. i.e. a sort-of anonymous trait.
impl I32 {
    fn p(self: &Self) {
        println!("p = {}", self.0);
    }
}

fn uses(i: &I32) {
    println!("uses = {:?}", i);
}

#[allow(clippy::unnecessary_operation)]
fn main() {
    let x = I32(1);
    //let y : &mut I32 = &mut x;
    //let y: &I32 = &mut x;
    let y: &I32 = &x;
    println!("Before uses");
    &x.p();
    x.p();
    uses(&x);
    uses(y);
    uses(y);
    println!("After uses");
    println!("End of main()");
}
