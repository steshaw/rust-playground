// This is a horrific experiment. Avert your eyes!

use std::iter::Peekable;
use std::str::Chars;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut p = string.chars().peekable();
    println!("peekable = {p:?}");
    let r = parse_exprs(&mut p);
    println!("brackets_are_balanced: parse_expr => r = {r:?}");
    println!("peekable at end = {p:?}");
    // No parse failures and we are at end-of-input
    r == Ok(()) && p.next() == None
}

fn parse_exprs(chars: &mut Peekable<Chars>) -> Result<(), String> {
    println!("parse_exprs: {:?}", chars);

    // While we are not at EOS, we can have a sequence of expressions.
    // {bracket-expressions}
    loop {
        let c = chars.peek();
        if c == None || c == Some(&')') || c == Some(&']') || c == Some(&'}') {
            break;
        }
        let r = parse_expr(chars);
        if r.is_err() { return r; }
    }

    Ok(())
}

fn parse_expr(chars: &mut Peekable<Chars>) -> Result<(), String> {
    println!("parse_expr: {:?}", chars);
    let c = chars.peek();
    println!("parse_expr c = {c:#?}");
    match c {
        None => Ok(()),
        Some('(') => {
            chars.next();
            parse_exprs(chars)?;
            let c = chars.peek();
            println!("parse_expr confirming match c = {c:?}");
            match c {
                Some(')') => {
                    chars.next();
                    Ok(())
                }
                _ => Err("Expected right round bracket".to_string()),
            }
        }
        Some('[') => {
            chars.next();
            parse_exprs(chars)?;
            let c = chars.peek();
            println!("parse_expr confirming match c = {c:?}");
            match c {
                Some(']') => {
                    chars.next();
                    Ok(())
                }
                _ => Err("Expected right square bracket".to_string()),
            }
        }
        Some('{') => {
            chars.next();
            parse_exprs(chars)?;
            let c = chars.peek();
            println!("parse_expr confirming match c = {c:?}");
            match c {
                Some('}') => {
                    chars.next();
                    Ok(())
                }
                _ => Err("Expected right curly bracket".to_string()),
            }
        }
        Some(')') => Ok(()),
        Some(']') => Ok(()),
        Some('}') => Ok(()),
        Some(_) => {
            chars.next();
            Ok(())
        }
    }
}
