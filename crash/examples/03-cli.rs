use std::env::args;

fn f1() {
    println!("f1");
    let mut args = args();
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
    println!("{:?}", args.next());
}
fn f2() {
    println!("f2");
    let mut args = args();
    loop {
        let a = args.next();
        if a.is_none() {
            break;
        };
        println!("{:?}", a);
    }
}
fn f3() {
    println!("f3");
    for a in args() {
        println!("{:?}", a);
    }
}
fn f4() {
    println!("f4");
    let v = args().collect::<Vec<_>>();
    for a in v {
        println!("{:?}", a);
    }
}
fn main() {
    let mut first = true;
    let fns : Vec<fn()> = vec![f1, f2, f3, f4];
    let fns = &fns;
//    let fns = fns.join(f1);
    for f in fns.iter() {
        if first {
            first = false
        } else {
            println!()
        }
        f();
    }
}
