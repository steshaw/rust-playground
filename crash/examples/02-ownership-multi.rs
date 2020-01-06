#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

#[allow(dead_code)]
fn uses_foobar(foobar: &Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x: Foobar = Foobar(1);
    let y: &Foobar = &x;
    println!("Before uses_foobar");
    uses_foobar(&x);
//    std::mem::drop(x);
    uses_foobar(y);
    println!("After uses_foobar");
}
