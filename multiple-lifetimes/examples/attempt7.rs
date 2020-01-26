//
// Modification of the following StackOverflow answer to remove the need for
// multiple lifetimes.
// https://stackoverflow.com/a/29862184/482382
//

static ZERO: i32 = 0;

struct Foo<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn get_x_or_zero_ref<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    if *x > *y {
        x
    } else {
        &ZERO
    }
}

fn main() {
    let x = 1;
    let v;
    let y = 2;
    let f = Foo { x: &x, y: &y };
    v = get_x_or_zero_ref(&f.x, &f.y);
    println!("{}", *v);
}
