use std::fmt;

fn print_first1<T : fmt::Display, E>(v : Vec<Result<T, E>>) -> () {
    if let Some(s) = v.iter().find_map(|f| f.as_ref().ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

fn print_first2<T : fmt::Display, E>(v : Vec<Result<T, E>>) -> () {
    if let Some(s) = v.into_iter().find_map(Result::ok) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

fn print_first3<T : fmt::Display, E>(v : Vec<Result<T, E>>) -> () {
    if let Some(Ok(s)) = v.iter().find(|f| f.is_ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

//macro print_first(v) { printfirst2(v) }

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
    print_first3(v);

    let errs : Vec<Result<char, &str>> = vec![Err("oops"), Err("argh")];

    print_first3(errs);
}
