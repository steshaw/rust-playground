pub fn factors(mut n: u64) -> Vec<u64> {
    // Imperative algorithm following Uncle Bob's kata.
    let mut result = Vec::new();
    let mut candidate = 2;
    while n > 1 {
        println!("candidate = {}", candidate);
        while n % candidate == 0 {
            result.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }
    result
}
