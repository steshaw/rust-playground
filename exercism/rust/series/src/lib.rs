pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec!["".to_string(); digits.len()]
    } else {
        digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|w| w.iter().collect()) // Convert vectors to String
            .collect()
    }
}
