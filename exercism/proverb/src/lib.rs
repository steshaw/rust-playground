pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        "".to_string()
    } else if list.len() == 2 {
        ["For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
        ].join("\n")
    } else if list.len() == 1 {
        "And all for the want of a ".to_string() + list[0] + "."
    } else {
        "Oh my!".to_string()
    }
}
