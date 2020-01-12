fn f1() {
    println!("f1");
    let mut args = std::env::args();
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
}
fn f2() {
    println!("f2");
    let mut args = std::env::args();
    while let a = args.next() {
        if a.is_none() {
            break;
        };
        println!("{:?}", a);
    }
}
fn main() {
    f1();
    println!();
    f2();
}
