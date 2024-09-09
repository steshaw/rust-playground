pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        "".to_string()
    } else {
        "And all for the want of a ".to_string() + list[0] + "."
    }
}
