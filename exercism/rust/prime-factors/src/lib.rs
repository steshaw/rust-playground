pub fn factors(mut n: u64) -> Vec<u64> {
    // Imperative algorithm following Uncle Bob's kata.
    let mut result = vec![];
    let mut candidate = 2;
    while n > 1 {
        while n % candidate == 0 {
            result.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }
    result
}
