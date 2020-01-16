#[derive(Debug, Clone)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self);
    }
}

fn uses(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn main() {
    let x = Foobar(0);
    let mut x1 = x.clone();
    x1.0 = 1;
    uses(x1);
    //uses(x1);
    println!("After uses");
    let mut x2 = x.clone();
    x2.0 = 2;
    uses(x2);
    println!("End of main");
}
