use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for f in factors.iter().filter(|&&f| f > 0) {
        for x in (*f..limit).step_by(*f as usize) {
            multiples.insert(x);
        }
    }
    multiples.into_iter().sum()
}
