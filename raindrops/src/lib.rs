/*
- If the number has 3 as a factor, output 'Pling'.
- If the number has 5 as a factor, output 'Plang'.
- If the number has 7 as a factor, output 'Plong'.
- If the number does not have 3, 5, or 7 as a factor,
  just pass the number's digits straight through.
*/

pub fn raindrops(i: i32) -> String {
    let f3 = i % 3 == 0;
    let f5 = i % 5 == 0;
    let f7 = i % 7 == 0;
    if f3 || f5 || f7 {
        let mut s = String::from("");
        if f3 { s.push_str("Pling") };
        if f5 { s.push_str("Plang") };
        if f7 { s.push_str("Plong") };
        s
    } else {
        i.to_string()
    }
}
