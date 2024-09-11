pub fn brackets_are_balanced(string: &str) -> bool {
    let mut foo = string.chars().filter(|ch|
        *ch == '[' || *ch == ']' || *ch == '{' || *ch == '}' || *ch == '[' || *ch == ']'
    ).collect::<String>();
    println!("foo = {foo:#?}");
    while foo.len() > 0 {
        println!("foo = {foo:#?}");
        let m = match (foo.chars().next().unwrap(), foo.chars().last().unwrap()) {
            ('[', ']') => true,
            ('{', '}') => true,
            ('(', ')') => true,
            _ => false,
        };
        if !m {
            return false;
        }
        // Delete matching pair
        foo = foo.as_str()[1..foo.len()-1].to_string();
        println!("foo (updated) = {foo:#?}");
    }
    true
}
