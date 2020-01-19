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
}

fn once() {
    println!("\nonce");
    let name = String::from("Alice");

    let welcome = || {
        let name = name; // Move the outer 'name' into the closure.
        println!("Welcome, {}", name);
    };

    //println!("after welcome name = {}", name); // Compile error: borrow of moved value: `name`.
    //welcome(); // Compile error at `call_once`: used of moved value: `welcome`.
    call_once(welcome);
    //println!("after welcome name = {}", name); // Compile error: borrow of moved value: `name`.
    //welcome(); // Compile error: use of moved value: `welcome`.
}

fn five_clone() {
    println!("\nfive_clone");
    let name = String::from("Alice");

    let welcome = || {
        let name = name.clone();
        println!("Welcome, {}", name);
    };
    println!("after welcome name = {}", name);
    welcome();
    call_five_times(welcome);
    println!("after welcome name = {}", name);
    welcome();
}

fn five_ref() {
    println!("\nfive_ref");
    let name = String::from("Alice");

    let welcome = || {
        let name = &name;
        println!("Welcome, {}", name);
    };
    println!("after welcome name = {}", name);
    welcome();
    call_five_times(welcome);
    println!("after welcome name = {}", name);
    welcome();
}

fn main() {
    once();
    five_clone();
    five_ref();
}
