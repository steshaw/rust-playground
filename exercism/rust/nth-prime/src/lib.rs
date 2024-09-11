struct Primes {
    primes: Vec<u32>,
    next: u32,
}

impl Primes {
    fn new() -> Self {
        Primes { primes: Vec::new(), next: 2 }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.next..).find(|n|
            self.primes.iter().all(|&prime| n % prime > 0)
        ).unwrap();
        self.primes.push(n);
        self.next = n + 1;
        Some(n)
    }
}

fn primes() -> Primes {
    Primes::new()
}

pub fn nth(n: u32) -> u32 {
    primes().nth(n as usize).unwrap()
}
