pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else {
        let mut iterations = 0u64;
        while n != 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            iterations += 1;
        }
        Some(iterations)
    }
}
