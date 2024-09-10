pub fn square(s: u32) -> u64 {
    if s >= 1 && s <= 64 {
        let mut num = 1;
        for _ in 1..s {
            num = num * 2
        }
        num
    } else {
        panic!("Invalid square: {s}")
    }
}

pub fn total() -> u64 {
    // A lot of repetitive calculation here...
    (1..=64).map(|n| square(n)).sum()
}
