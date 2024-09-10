/*
    9 is an Armstrong number, because 9 = 9^1 = 9
    10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
    153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
    154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
 */
pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let pow = s.len();
    println!("num = {num}, s = {s}, pow = {}", pow);
    let pow = s.len();
    s.chars().map(|c|
        c.to_digit(10).unwrap().pow(pow as u32)
    ).sum::<u32>() == num
}
