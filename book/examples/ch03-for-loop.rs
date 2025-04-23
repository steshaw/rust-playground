fn iter_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn iter_with_while_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Use `len` instead of repeating the size of the array.
    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn iter_with_for_by_index() {
    let a = [10, 20, 30, 40, 50];

    // Use `len` instead of repeating the size of the array.
    for index in 0..a.len() {
        println!("the value is: {}", a[index]);
    }
}
fn iter_with_idiomatic_for() {
    let a = [10, 20, 30, 40, 50];

    for v in a {
        println!("the value is: {}", v);
    }
}
fn main() {
    iter_with_while();
    println!();
    iter_with_while_2();
    println!();
    iter_with_for_by_index();
    println!();
    iter_with_idiomatic_for();
}
