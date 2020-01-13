mod intersperse;
use crate::intersperse::*;

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
    if false {
        let sep_line = || println!();
        let fns: Vec<fn()> = vec![f1, f2, f3, f4];
        let fns = intersperse_vec(sep_line as fn(), fns);
        for f in fns {
            f();
        }
    }

    let fns = [f1, f2, f3, f4];
    // FIXME: Can the horrors of this be addressed?
    for f in fns.iter().intersperse(&((|| println!()) as fn())) {
        f();
    }
}
