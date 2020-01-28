static ZERO: i32 = 0;

#[derive(Debug)]
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> Foo<'a, 'b> {
    if *x > *y {
        Foo { x, y }
    } else {
        Foo { x, y: &ZERO }
    }
}

fn main() {
    let x = 1;
    let y = 2;
    let v = get_x_or_zero_ref(&x, &y);
    println!("{:?}", v);
}
