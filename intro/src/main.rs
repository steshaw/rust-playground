use async_std;
use num_format::{Locale, ToFormattedString};

async fn hello(even_odd: &str, i: i32) {
    let answer = 1_234_5;
    println!(
        "{}: Hello, {} {}",
        i,
        even_odd,
        answer.to_formatted_string(&Locale::en)
    );
    assert_eq!(answer, 12_345);
}

async fn hellos() {
    for i in 1..=3 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        hello(even_odd, i).await;
    }
}

fn sum() {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}

#[async_std::main]
async fn main() {
    hellos().await;
    sum();
}
