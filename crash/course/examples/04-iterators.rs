#![feature(bool_to_option)]

struct Empty;

impl Iterator for Empty {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct UltimateAnswers;

impl Iterator for UltimateAnswers {
    type Item = u8;
    fn next(self: &mut Self) -> Option<u8> {
        Some(42)
    }
}

struct Count{current: u32, to: u32}

impl Count {
    fn new(to: u32) -> Count {
        Count{current : 0, to}
    }
}

impl Iterator for Count {
    type Item = u32;
    fn next(& mut self) -> Option<Self::Item> {
        (self.current < self.to).then(|| {
            self.current += 1;
            self.current
        })
    }
}

struct Fibonacci{state : (u128, u128)}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci{ state : (0, 1)}
    }
}

impl Iterator for Fibonacci {
    type Item = u128;
    fn next(& mut self) -> Option<Self::Item> {
        let result = Some(self.state.0);
        self.state = (self.state.1, self.state.0 + self.state.1);
        result
    }
}

fn main() {
    for _ in Empty {
        panic!("Argh, an element from Empty!");
    }
    println!("Done with Empty!");
    println!();

    println!("The answer to the ultimate question of life, the universe and everything:");
    for answer in UltimateAnswers.take(5) {
        println!("{}", answer);
    }
    println!("Done with UltimateAnswers!");

    println!();
    println!("1 to 10");
    for n in Count::new(10) {
        println!("{}", n);
    }
    println!("Done with Count!");

    println!();
    let fibs = Fibonacci::new().take(10).collect::<Vec<_>>();
    println!("fibs = {:?}", fibs);
}
