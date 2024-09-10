use std::iter::once;

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut candidates = once(2).chain((3..).step_by(2));
    while n > 1 {
        let candidate = candidates.next().unwrap();
        while n % candidate == 0 {
            result.push(candidate);
            n /= candidate;
        }
    }
    result
}
