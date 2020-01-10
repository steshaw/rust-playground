#[derive(Debug)]
struct Foobar(i32);

fn main() {
    let mut x = Foobar(1);

    x.0 = 2;

    println!("{:?}", x);
}
