use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let inclusive = 1..=100;
    for i in inclusive.clone() {
        println!("Range includes {i}");
    }
    let secret_number = rand::thread_rng().gen_range(inclusive);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
