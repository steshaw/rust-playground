#[derive(Debug)]
struct Foobar(i32);

fn double(mut i: Foobar) -> Foobar {
    i.0 *= 2;
    i
}

fn main() {
    let x: Foobar = Foobar(1);
    println!("x = {}", x.0);
    let y: Foobar = double(x);
//    println!("x = {}", x.0);
    println!("y = {}", y.0);
}
