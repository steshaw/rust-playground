fn main() {
    // owned by main
    let name_outer = String::from("Alice");

    let say_hi = || {
        // force a move, again, we'll get smarter in a second
        let name_inner = name_outer;
        println!("Hello, {}", name_inner);
    };

    // main no longer owns name_outer, try this:
    //println!("Using name from main: {}", name_outer); // error!

    // but name_inner lives on, in say_hi!
    say_hi(); // success
    //say_hi(); // error!
}
