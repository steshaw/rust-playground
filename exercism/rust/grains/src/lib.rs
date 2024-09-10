pub fn square(s: u32) -> u64 {
    if s >= 1 && s <= 64 {
        2u64.pow(s - 1)
    } else {
        panic!("Invalid square: {s}")
    }
}

pub fn total() -> u64 {
    (2u128.pow(64) - 1) as u64
}
