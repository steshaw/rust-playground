use std::ops;

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
    current: u8,
    to: u8,
}

impl Count {
    fn new(to: u8) -> Count {
        Count { current: 0, to }
    }
}

trait When<T, F>
where
    F: FnOnce() -> T,
{
    fn when(self, f: F) -> Option<T>;
}

impl<T, F> When<T, F> for bool
where
    F: FnOnce() -> T,
{
    fn when(self, f: F) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

impl Iterator for Count {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        (self.current < self.to).when(|| {
            self.current += 1; // FIXME: Potential overflow.
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
    I: Iterator,
    I::Item: ops::Add<Output = I::Item>,
    I::Item: From<u8>,
    I::Item: Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| n + n)
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
    println!("Count(10): {:?}", Count::new(10).collect::<Vec<_>>());
    let one_to_ten = Count::new(10);
    for n in one_to_ten {
        println!("n = {}", n);
    }
    let mut one_to_ten = Count::new(10);
    for i in u8::min_value()..=u8::max_value() {
        println!("{}: next() => {:?}", i, one_to_ten.next());
    }

    println!();
    let fibs = Fibonacci::new().take(10).collect::<Vec<_>>();
    println!("fibs = {:?}", fibs);

    println!();
    let orig_iter = 1..11u64;
    let doubled_iter = Doubler(orig_iter);
    println!("doubles = {:?}", doubled_iter.collect::<Vec<_>>());

    println!();
    let doubles = (1..11).map(|n| n + 2).collect::<Vec<_>>();
    println!("idiomatic doubles = {:?}", doubles);

    println!();
    for i in (1..11)
        .skip(3)
        .map(|x| x + 1)
        .inspect(|i| println!("inspected: {}", i))
        .filter(|x| x % 2 == 0)
    {
        println!("{}", i);
    }

    println!("sum 1 to 10 sum : {}", (1..=10).sum::<u16>());
    println!("sum 1 to 10 fold: {}", (1..=10).fold(0, |acc, n| acc + n));
}
