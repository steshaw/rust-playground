//
// Inspired by https://stackoverflow.com/a/33217302/482382
//

fn main() {
    let strs: Vec<&str> = vec!["one", "two", "three"];
    let strings: Vec<String> = strs.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

    let _strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    let _strs: Vec<&str> = strings.iter().map(String::as_str).collect();
    let _strs: Vec<&str> = strings.iter().map(|s| s.as_ref()).collect();
    let _strs: Vec<&str> = strings.iter().map(|s| s.as_ref()).collect();

    let _strs: Vec<&str> = strings
        .iter()
        .map(|s| {
            let s: &str = s;
            s
        })
        .collect();

    let _strs: Vec<&str> = strings.iter().map(to_str_1).collect();
    let _strs: Vec<&str> = strings.iter().map(to_str_2).collect();

    let _strs: Vec<&str> = strings
        .iter()
        .map(|s| {
            let s: &str = s;
            s
        })
        .collect();
    let _strs: Vec<&str> = strings
        .iter()
        .map(|s: &String| {
            let s: &str = &s;
            s
        })
        .collect();

    let c: fn(&String) -> &str = |s| s;
    let _strs: Vec<&str> = strings.iter().map(c).collect();

    let _strs: Vec<&str> = strings.iter().map(|s| s as &str).collect();

    use std::ops::Deref;
    let _strs: Vec<&str> = strings.iter().map(Deref::deref).collect();
    let _strs: Vec<&str> = strings.iter().map(|s| s.deref()).collect();
    let _str: Vec<&str> = strings.iter().map(|s: &String| (*s).deref()).collect();
    let _str: Vec<&str> = strings.iter().map(|s: &String| &**s).collect();
}

#[allow(clippy::needless_lifetimes, clippy::ptr_arg)]
fn to_str_1<'a>(s: &'a String) -> &'a str {
    s
}

#[allow(clippy::ptr_arg)]
fn to_str_2(s: &String) -> &str {
    s
}
