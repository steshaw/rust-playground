use std::fmt;

/*
fn print_first<T : fmt::Display, E>(v : Vec<Result<T, E>>) -> () {
    if let Some(s) = v.iter().find_map(|f| f.ok()) {
        println!("Found {}", s);
    }
}
*/
fn print_first<T : fmt::Display, E>(v : Vec<Result<T, E>>) -> () {
    if let Some(s) = v.into_iter().find_map(|f| f.ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

#[test]
fn find_it() {
    let v = vec![
        Err("Argh"),
        Err("Heck"),
        Err("Bugger"),
        Err("Squirm"),
        Ok(42),
        Err("Another"),
        Err("Penultimate"),
        Err("Ultimate"),
    ];

    if let Some(s) = v.iter().find_map(|f| f.ok()) {
        println!("Found {}", s);
    }
    print_first(v);

    let errs : Vec<Result<char, &str>> = vec![Err("oops"), Err("argh")];

    print_first(errs);
}
