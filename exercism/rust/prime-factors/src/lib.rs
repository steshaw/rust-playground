pub fn factors(n: u64) -> Vec<u64> {
    let mut m = n;
    let mut primes = vec![];
    let mut candidate = 2;
    while m > 1 {
        while m % candidate == 0 {
            primes.push(candidate);
            m /= candidate;
        }
        candidate += 1;
    }
    primes
}
