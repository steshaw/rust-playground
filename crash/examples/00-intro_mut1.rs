fn main() {
    let i: isize = 1;
    let j: isize = foo(i);
    println!("i = {}", i);
    println!("j = {}", j);
}

fn foo(mut i: isize) -> isize {
    i += 1;
    println!("i = {}", i);
    i
}
