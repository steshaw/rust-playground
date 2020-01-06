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

fn main() {
    fn usage() -> ! {
        panic!("usage: fizzbuzz <num>")
    }
    let args = std::env::args();
    if args.len() != 2 {
        usage();
    }
    #[allow(clippy::identity_conversion)]
    let a1 = args.into_iter().nth(1);
    let fb = match a1 {
        Some(s) if s == "1" => fizzbuzz1,
        Some(s) if s == "2" => fizzbuzz2,
        Some(s) if s == "3" => fizzbuzz3,
        _ => usage()
    };
    for i in 1..=100 {
        fb(i);
    }
}
