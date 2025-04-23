use std::cmp::Ordering;
use rand::Rng;
use std::io;

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

    //let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.trim().parse::<u32>() {
        Ok(guess) => {
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => println!("You win!"),
            }
        },
        Err(err) => { println!("Please input a number! {err}") },
    }
}
