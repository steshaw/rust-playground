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
#[allow(clippy::while_let_on_iterator)]
fn f2() {
    println!("f2");
    let mut args = args();
    while let Some(a) = args.next() {
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
fn f5() {
    println!("f5");
    args().for_each(|a| println!("{:?}", a));
}
fn main() {
    let fns = [f1, f2, f3, f4, f5];

    if false {
        let sep_line = || println!();
        let fns: Vec<fn()> = fns.to_vec();
        let fns = intersperse_vec(sep_line as fn(), fns);
        for f in fns {
            f();
        }
    }

    // FIXME: Fix the horrors?
    for f in fns.iter().intersperse(&((|| println!()) as fn())) {
        f();
    }
}
