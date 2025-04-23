fn main() {
    let mut counter = 0;

    // Turns out that loop can be an expression!
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
