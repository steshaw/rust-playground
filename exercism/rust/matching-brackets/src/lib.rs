use std::collections::HashSet;

pub fn brackets_are_balanced(string: &str) -> bool {
    let bracket_chars = vec!['[', ']', '{', '}', '(', ')']
        .into_iter()
        .collect::<HashSet<_>>();
    // Remove all characters but the brackets.
    let mut brackets = string
        .chars()
        .filter(|ch| bracket_chars.contains(&ch))
        .collect::<String>();
    while brackets.len() > 0 {
        let l = brackets.len();
        brackets = brackets.replace("[]", "");
        brackets = brackets.replace("()", "");
        brackets = brackets.replace("{}", "");
        if brackets.len() == l {
            break;
        }
    }
    brackets.len() == 0
}
