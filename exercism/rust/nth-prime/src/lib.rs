pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 11,
        5 => 13,
        _ => todo!("Not implemented nth({n})")
    }
}
