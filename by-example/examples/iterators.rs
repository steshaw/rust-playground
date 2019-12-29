use std::fmt;

#[allow(clippy::into_iter_on_ref)]
fn main() {
    let v1 = vec![1, 2, 3];

    v1.iter().for_each(|i| println!("{:?}", i));
    (&v1).iter().for_each(|i| println!("{:?}", i));
    (&v1).into_iter().for_each(|i| println!("{:?}", i));

    println!("{}", v1.iter().any(|&i| i == 3));
    println!("{}", (&v1).iter().any(|i| *i == 3));
    println!("{}", (&v1).into_iter().any(|i| *i == 3));

    for i in v1 {
        println!("i = {}", i);
    }

    let v2 = vec![1, 2, 3];
    println!("find => {:?}", v2.iter().find(|&&i| i == 1));

    let v3 = vec![Err("argh"), Ok(42), Err("x"), Err("y")];
    println!("find => {:?}", v3.iter().find(|&&a| a.is_ok()));
    print_first0(v3.clone());
    print_first1(&v3);
    print_first2(&v3);
    print_first3(v3);
}

fn print_first0<T, E>(v: Vec<Result<T, E>>)
where
    T: fmt::Display,
    E: fmt::Display,
{
    if let Some(Ok(s)) = v.iter().find(|i| i.is_ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

fn print_first1<T: fmt::Display, E>(v: &Vec<Result<T, E>>) -> () {
    if let Some(s) = v.iter().find_map(|i| i.as_ref().ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

fn print_first2<T: fmt::Display, E>(v: &Vec<Result<T, E>>) -> () {
    if let Some(Ok(s)) = v.iter().find(|i| i.is_ok()) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}

fn print_first3<T: fmt::Display, E>(v: Vec<Result<T, E>>) -> () {
    if let Some(s) = v.into_iter().find_map(Result::ok) {
        println!("Found {}", s);
    } else {
        println!("No okays");
    }
}
