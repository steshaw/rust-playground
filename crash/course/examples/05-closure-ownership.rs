fn main() {
    let say_hi = {
        // Creation of smaller scope.
        // Owned by smaller scope.
        let name_outer = String::from("Alice");

        || {
            // Use by reference.
            let name_inner = &name_outer; // Compile error: `name_outer` does not live long enough.
            println!("Hello, {}", name_inner);
        }
    };

    // Compile error: cannot find value `name_outer` in this scope.
    //println!("Using name from main: {}", name_outer);

    say_hi();
    say_hi();
}
