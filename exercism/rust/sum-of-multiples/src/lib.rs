use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();
    for f in factors {
        if *f > 0 {
            for x in (*f..limit).step_by(*f as usize) {
                multiples.push(x);
            }
        }
    }
    multiples.iter().unique().sum()
}
