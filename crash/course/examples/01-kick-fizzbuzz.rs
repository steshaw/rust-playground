fn fizzbuzz1(i: u8) {
    let divisible_by = |n| i % n == 0;
    let div_3 = divisible_by(3);
    let div_5 = divisible_by(5);
    if div_3 && div_5 {
        println!("fizzbuzz");
    } else if div_3 {
        println!("fizz");
    } else if div_5 {
        println!("buzz");
    } else {
        println!("{}", i);
    }
}

fn fizzbuzz2(i: u8) {
    match (i % 3, i % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        (_, _) => println!("{}", i),
    }
}

fn fizzbuzz3(i: u8) {
    fn fb(i: u8) -> String {
        match (i % 3, i % 5) {
            (0, 0) => "fizzbuzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            (_, _) => format!("{}", i),
        }
    }
    println!("{}", fb(i));
}

fn fizzbuzz4() {
    fn fb(i: u8) -> String {
        match (i % 3, i % 5) {
            (0, 0) => "fizzbuzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            (_, _) => format!("{}", i),
        }
    }
    (1..=100).for_each(|i| println!("{}", fb(i)))
}

fn fizzbuzz5() {
    fn fb(i: u8) -> String {
        match (i % 3, i % 5) {
            (0, 0) => "fizzbuzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            (_, _) => format!("{}", i),
        }
    }
    for s in (1..=100).map(fb) {
        println!("{}", s);
    }
}

fn fizzbuzz6() {
    use std::fmt;
    struct FizzBuzzer(u8);
    impl fmt::Display for FizzBuzzer {
        fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let FizzBuzzer(i) = self;
            match (i % 3, i % 5) {
                (0, 0) => write!(fmt, "fizzbuzz"),
                (0, _) => write!(fmt, "fizz"),
                (_, 0) => write!(fmt, "buzz"),
                (_, _) => write!(fmt, "{}", i),
            }
        }
    }
    for fizzbuzzer in (1..=100).map(FizzBuzzer) {
        println!("{}", fizzbuzzer);
    }
}

fn main() {
    fn fb(f: fn(u8) -> ()) {
        for i in 1..=100 {
            f(i);
        }
    }
    fn usage() -> ! {
        panic!("usage: fizzbuzz <num>")
    }
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        usage();
    }
    #[allow(clippy::identity_conversion)]
    let a1 = &args[1]; // NOTE: Indexing protect by len check.
    match a1 {
        s if s == "1" => fb(fizzbuzz1),
        s if s == "2" => fb(fizzbuzz2),
        s if s == "3" => fb(fizzbuzz3),
        s if s == "4" => fizzbuzz4(),
        s if s == "5" => fizzbuzz5(),
        s if s == "6" => fizzbuzz6(),
        _ => usage(),
    }
}
