pub fn factors(n: u64) -> Vec<u64> {
    if n > 1 {
        let mut m = n;
        let mut primes = vec![];
        while m % 2 == 0 {
            primes.push(2);
            m /= 2;
        }
        if m > 1 {
            primes.push(m);
        }
        primes
    } else {
        vec![]
    }
}
