pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len())
        .map(|n| {
            digits
                .to_string()
                .chars()
                .skip(n)
                .take(len)
                .collect::<String>()
        })
        .filter(|s| s.len() == len)
        .collect::<Vec<_>>()
}
