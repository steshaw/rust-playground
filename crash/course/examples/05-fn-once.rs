fn call_five_times<F>(f: F)
where
    F: Fn() -> (),
{
    for _ in 1..=5 {
        f()
    }
    //(1..=5).for_each(|_| f() );
}

fn call_once<F>(f: F)
where
    F: FnOnce() -> (),
{
    f()
    //(1..=5).for_each(|_| f() );
}

fn main() {
    let name = String::from("Alice");

    let welcome = || {
        let mut name = name; // here's the magic
        name += " and Bob";
        println!("Welcome, {}", name);
    };

    //welcome();
    //println!("before welcome name = {}", name1);

    //call_five_times(welcome);
    call_once(welcome);

    //println!("after welcome name = {}", name1);
}
