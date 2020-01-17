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
    fn true_as_some(self, f: F) -> Option<T>;
}

impl<T, F> When<T, F> for bool
where
    F: FnOnce() -> T,
{
    fn true_as_some(self, f: F) -> Option<T> {
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
        (self.current < self.to).true_as_some(|| {
            self.current += 1;
            self.current
        })
    }
}

struct Fibonacci0(u8, u8);
impl Fibonacci0 {
    fn new() -> Fibonacci0 {
        Fibonacci0(0, 1)
    }
}
impl Iterator for Fibonacci0 {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0;
        let next = self.0.checked_add(self.1);
        match next {
            None => None,
            Some(next) => {
                *self = Fibonacci0(self.1, next);
                Some(result)
            }
        }
    }
}

enum Fibonacci {
    Running { state: (u8, u8) },
    Last(u8),
    Done,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci::Running { state: (0, 1) }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use Fibonacci::*;
        match self {
            Running { state } => {
                let result = state.0;
                let next = state.0.checked_add(state.1);
                match next {
                    None => {
                        *self = Last(state.1);
                    }
                    Some(next) => {
                        *self = Running {
                            state: (state.1, next),
                        };
                    }
                }
                Some(result)
            }
            Last(n) => {
                let result = Some(*n);
                *self = Done;
                result
            }
            Done => None,
        }
    }
}

struct Doubler<I>(I);

impl<I> Iterator for Doubler<I>
where
    I: Iterator,
    I::Item: ops::Add<Output = I::Item>,
    I::Item: Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| n + n)
    }
}

fn sum<I>(from : I::Item, iter: I) -> I::Item
where
    I: Iterator,
    I::Item: ops::Add<Output = I::Item>,
{
    iter.fold(from, ops::Add::add)
}

#[allow(clippy::unnecessary_fold)]
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
    let fibs = Fibonacci0::new().take(20).collect::<Vec<_>>();
    println!("fibs0 = {:?}", fibs);

    println!();
    let fibs = Fibonacci::new().take(20).collect::<Vec<_>>();
    println!("fibs1 = {:?}", fibs);

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

    println!("sum 1 to 10, .sum():     {}", (1..=10).sum::<u8>());
    println!(
        "sum 1 to 10, fold fn:    {}",
        (1..=10).fold(0, |acc, n| acc + n)
    );
    println!(
        "sum 1 to 10, fold add:   {}",
        (1..=10).fold(0, ops::Add::add)
    );
    println!("sum 1 to 10, sum   u8:        {}", sum(0, 1..=10u8));
    println!("sum 1 to 10, sum  u16:        {}", sum(0, 1..=10u16));
    println!("sum 1 to 10, sum  u32:        {}", sum(0, 1..=10u32));
    println!("sum 1 to 10, sum u128:        {}", sum(0, 1..=10u128));
    println!("sum 1 to 10, sum   i8:        {}", sum(0, 1..=10i8));
    println!("sum 1 to 10, sum  i16:        {}", sum(0, 1..=10i16));
    println!("sum 1 to 10, sum  i32:        {}", sum(0, 1..=10i32));
    println!("sum 1 to 10, sum i128:        {}", sum(0, 1..=10i128));
}
