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

struct Count {
    current: u32,
    to: u32,
}

impl Count {
    fn new(to: u32) -> Count {
        Count { current: 0, to }
    }
}

impl Iterator for Count {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        (self.current < self.to).then(|| {
            self.current += 1;
            self.current
        })
    }
}

struct Fibonacci {
    state: (u128, u128),
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { state: (0, 1) }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.state.0);
        self.state = (self.state.1, self.state.0 + self.state.1);
        result
    }
}

struct Doubler<I>(I);

impl<I> Iterator for Doubler<I>
where
    I: Iterator<Item = u32>,
{
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.0.next().map(|n| n * 2)
    }
}

fn main() {
    println!("Empties = {:?}", Empty.collect::<Vec<_>>());

    println!();
    println!(
        "Ultimate answers = {:?}",
        UltimateAnswers.take(5).collect::<Vec<_>>()
    );

    println!();
    println!("1..10: {:?}", Count::new(10).collect::<Vec<_>>());

    println!();
    let fibs = Fibonacci::new().take(10).collect::<Vec<_>>();
    println!("fibs = {:?}", fibs);

    println!();
    let orig_iter = 1..11;
    let doubled_iter = Doubler(orig_iter);
    println!("doubles = {:?}", doubled_iter.collect::<Vec<_>>());
}
