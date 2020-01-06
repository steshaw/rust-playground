//
// From https://gist.github.com/elfsternberg/4757b918d034e5a1820704af6b94d82d
//

use std::fmt;

enum FizzBuzzOr {
    Fizzbuzz(&'static str),
    Or(i32),
}

impl fmt::Display for FizzBuzzOr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FizzBuzzOr::*;
        match self {
            Fizzbuzz(a) => write!(f, "{}", a),
            Or(i) => write!(f, "{}", i),
        }
    }
}

fn fizzbuzz(i: i32) -> FizzBuzzOr {
    match (i % 3, i % 5) {
        // The order is important; the more precise definitions
        // must come before those that use wildcards, or the
        // wildcarded definitions will pick up incorrect
        // answers.
        (0, 0) => FizzBuzzOr::Fizzbuzz("fizzbuzz"),
        (0, _) => FizzBuzzOr::Fizzbuzz("fizz"),
        (_, 0) => FizzBuzzOr::Fizzbuzz("buzz"),
        (_, _) => FizzBuzzOr::Or(i),
    }
}

fn main() {
    for i in 1..100 {
        println!("{}", fizzbuzz(i));
    }
}
