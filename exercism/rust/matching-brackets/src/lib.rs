use std::collections::HashSet;

pub fn brackets_are_balanced(string: &str) -> bool {
    let bracket_chars = vec!['[', ']', '{', '}', '(', ')']
        .into_iter()
        .collect::<HashSet<_>>();
    let mut brackets = string.to_string();
    // Remove all characters but the brackets.
    brackets.retain(|c| bracket_chars.contains(&c));
    while brackets.len() > 0 {
        let l = brackets.len();
        brackets = brackets
            .replace("[]", "")
            .replace("()", "")
            .replace("{}", "");
        if brackets.len() == l {
            break; // Bail if we didn't reduce
        }
    }
    brackets.is_empty()
}
