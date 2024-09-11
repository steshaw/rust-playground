use std::str::Chars;

pub fn brackets_are_balanced(string: &str) -> bool {
    parse_brackets(&mut string.chars()) == Ok(None)
}

fn parse_brackets(chars: &mut Chars) -> Result<Option<char>, String> {
    let c = chars.next();
    println!("parse_brackets c = {c:#?}");
    match c {
        None => Ok(None),
        Some('(') => {
            let r = parse_brackets(chars);
            match r {
                Ok(Some(')')) => parse_brackets(chars),
                _ => Err("Expected right round bracket".to_string()),
            }
        }
        Some('[') => {
            let r = parse_brackets(chars);
            match r {
                Ok(Some(']')) => parse_brackets(chars),
                _ => Err("Expected right square bracket".to_string()),
            }
        }
        Some('{') => {
            let r = parse_brackets(chars);
            match r {
                Ok(Some('}')) => parse_brackets(chars),
                _ => Err("Expected right curly bracket".to_string()),
            }
        }
        Some(')') => Ok(c),
        Some(']') => Ok(c),
        Some('}') => Ok(c),
        Some(_) => {
            // Keep going
            parse_brackets(chars)
        }
    }
}
