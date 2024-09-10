pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let pow = s.len();
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(pow as u32))
        .sum::<u32>()
        == num
}
