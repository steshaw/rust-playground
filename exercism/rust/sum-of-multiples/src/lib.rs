use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();
    for f in factors {
        println!("factor is {f}");
        let mut n: u32 = *f;
        if n > 0 {
            while n < limit {
                println!("n is {n}");
                multiples.push(n);

                n += f;
            }
        }
    }
    multiples.iter().unique().sum()
}
