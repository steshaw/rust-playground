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
        println!("uses I32: {}", self.0);
    }
}

fn main() {
    let x = I32(1);
    println!("Before uses");
    x.p();
    x.p();
    println!("After uses");
    println!("End of main()");
}
