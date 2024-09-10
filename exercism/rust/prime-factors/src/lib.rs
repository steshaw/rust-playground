pub fn factors(n: u64) -> Vec<u64> {
    let mut m = n;
    let mut primes = vec![];
    let candidate = 2;
    while m % candidate == 0 {
        primes.push(candidate);
        m /= candidate;
    }
    if m > 1 {
        primes.push(m);
    }
    primes
}
