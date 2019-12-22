use async_std;
use std;
use num_format::{Locale, ToFormattedString};

async fn hello(even_odd: &str, i: i32) {
    let answer = 12_345;
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

fn sum_f64() {
    let mut sum = 0.1;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum_f64 is {}", sum);
}

fn sqr(x: f64) -> f64 {
    x * x
}

fn sqr_it() {
    let r = sqr(2_f64);
    println!("square is {}", r);
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn print_factorials() {
    for n in 0..5 {
        println!("facorial({}) => {}", n, factorial(n));
    }
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn use_by_ref() {
    let i = 10;
    let r1 = by_ref(&i);
    let r2 = by_ref(&41);
    println!("by_refs {} {}", r1, r2)
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn use_modifies() {
    let mut r = 0.0;
    println!("before: r = {}", r);
    modifies(&mut r);
    println!(" after: r = {}", r);
}

fn cosine() {
    let x = 2.0 * std::f64::consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();
    let r = abs_difference;
    println!("r = {}", r);

    assert!(abs_difference < 1e-10);
}

fn maths() {
    for a in (-5..=5).map(|i| {i as f64}) {
      println!("abs({}) = {}", a, abs(a));
    }
    for a in 1..=15 {
      for (a, from, to) in [(a as f64, 5.0, 10.0), (a as f64, -3.0, 1.0)].iter() {
        println!("clamp({}, {}, {}) = {}", a, from, to, clamp(*a, *from, *to));
      }
    }
}

#[async_std::main]
async fn main() {
    hellos().await;
    sum();
    sum_f64();
    sqr_it();
    print_factorials();
    use_by_ref();
    use_modifies();
    maths();
    cosine();
}
