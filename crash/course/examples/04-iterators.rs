struct Empty;

impl Iterator for Empty {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct UltimateAnswers;

impl Iterator for UltimateAnswers {
    type Item = u8;
    fn next(self: &mut Self) -> Option<u8> {
        Some(42)
    }
}

fn main() {
    for _ in Empty {
        panic!("Argh, an element from Empty!");
    }
    println!("Done with Empty!");

    println!("The answer to the ultimate question of life, the universe and everything:");
    for answer in UltimateAnswers.take(5) {
        println!("{}", answer);
    }
    println!("Done with UltimateAnswers!");
}
