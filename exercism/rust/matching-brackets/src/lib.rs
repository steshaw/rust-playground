pub fn brackets_are_balanced(string: &str) -> bool {
    // Remove all characters but the brackets.
    let mut brackets = string
        .chars()
        .filter(|ch| {
            *ch == '['
                || *ch == ']'
                || *ch == '{'
                || *ch == '}'
                || *ch == '('
                || *ch == ')'
        })
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
