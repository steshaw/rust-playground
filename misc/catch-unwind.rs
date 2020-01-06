use std::any::Any;

//#![allow(unused)]
fn main() {
    use std::panic::catch_unwind;

    println!("No panic");
    let result = catch_unwind(|| {
        println!("All good!");
    });
    assert!(result.is_ok());
    println!();

    println!("Explicit panic");
    let result = catch_unwind(|| {
        panic!("yikes!");
    });
    assert!(result.is_err());
    print_recovered(result);
    println!();

    println!("Implicit panic");
    let result = catch_unwind(|| {
        None::<u8>.unwrap();
    });
    assert!(result.is_err());
    print_recovered(result);
}

fn print_recovered(result: Result<(), Box<dyn Any + Send>>) {
    let r = result.map_err(|e| format!("{:?}", e.type_id()));
    println!("Recovered from panic, result is {:?}", r);
}
