#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

#[derive(Copy, Clone, Debug)]
struct Copyable(i32);

#[allow(dead_code)]
fn uses_foobar(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

fn drop_now<T: Drop>(_x: T) {}

fn main() {
    let x = Foobar(1);
    #[allow(unused_variables)]
    let copyable = Copyable(1);
    drop_now(x);
    /*
    drop_now(copyable);
    drop_now(copyable);
    */
    println!("Before uses_foobar");
    /*
    uses_foobar(x);
    */
    println!("After uses_foobar");
}
