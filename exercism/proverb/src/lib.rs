use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        "".to_string()
    } else if list.len() == 1 {
        "And all for the want of a ".to_string() + list[0] + "."
    } else if list.len() == 2 {
        [
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n")
    } else {
        let r = list
            .iter()
            .zip(list.iter().skip(1))
            .map(|words| format!("For want of a {} the {} was lost.", words.0, words.1))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{r}\nAnd all for the want of a {}.", list[0])
    }
}
