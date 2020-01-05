fn main() {
    let mut i: isize = 1;
    let mut j: isize = 2;
    foo(&mut i, &mut j);
    println!("main");
    println!("i = {}", i);
    println!("j = {}", j);
}

fn foo<'a>(mut i: &'a mut isize, j: &'a mut isize) {
    *i *= 10;
    println!("foo1");
    println!("i = {}", i);
    println!("j = {}", j);
    i = j;
    println!("foo2");
    println!("i = {}", i);
    *i *= 10;
    println!("foo3");
    println!("i = {}", i);
}

