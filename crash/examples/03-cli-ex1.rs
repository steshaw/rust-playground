use std::env::{args, Args};
use std::iter::Skip;

fn main() {
    let args: Skip<Args> = args().skip(1);
    for arg in args {
        println!("{}", arg);
    }
}
