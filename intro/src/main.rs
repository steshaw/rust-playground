use async_std;
use num_format::{Locale, ToFormattedString};
use std::f64;

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

fn sum_i32() {
    fn sum_1_to_5() -> i32 {
      let mut sum = 0;
      for i in 0..5 {
          sum += i;
      }
      sum
    }
    let sum = sum_1_to_5();
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
    let x = 2.0_f64 * f64::consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();
    let r = abs_difference;
    println!("r = {}", r);

    assert!(abs_difference < 1e-10);
}

fn maths() {
    for a in (-5..=5).map(|i| i as f64) {
        println!("abs({}) = {}", a, abs(a));
    }
    for a in 1..=15 {
        for (a, from, to) in [(a as f64, 5.0, 10.0), (a as f64, -3.0, 1.0)].iter() {
            println!("clamp({}, {}, {}) = {}", a, from, to, clamp(*a, *from, *to));
        }
    }
}

fn arrays() {
    let arr = [10, 20, 30, 40];
    println!("length = {}", arr.len());

    let first = arr[0];
    println!("first = {}", first);

    // Panics.
    if false {
        // Avoid 'out of bounds'!
        let i = 12;
        println!("arr[{}] => {}", i, arr[i]);
    }

    for i in 0..4 {
        println!("arr[{}] => {}", i, arr[i]);
    }

    if false {
        // Avoid 'out of bounds'!
        for i in 0..5 {
            println!("arr[{}] => {}", i, arr[i]);
        }
    }
}

fn sum(values: &[i32]) -> i32 {
    let mut r = 0;
    for i in 0..values.len() {
        r += values[i];
    }
    r
}

fn sum_array() {
    let is = [10,20,30,40];
    let s = sum(&is);
    println!("s => {}", s);
}

fn print_arrays() {
    {
        let ints = [1, 2, 3];
        println!("ints {:?}", ints);
        let slice1 = &ints[0..2];
        println!("slice1 {:?}", slice1);
        let slice2 = &ints[1..];
        println!("slice2 {:?}", slice2);
        if false { // Avoid 'index out of bounds'
            let slice3 = &ints[2..5];
            println!("slice3 {:?}", slice3);
        }
    }
    {
        let floats = [1.1, 2.1, 3.1];
        println!("floats {:?}", floats);
    }
    {
        let strings = ["hello", "world"];
        println!("strings {:?}", strings);
    }
    {
        let ints_ints = [[1, 2], [10, 20]];
        println!("ints_ints {:?}", ints_ints);
    }
}

fn option_arrays() {
    let ints = [1, 2, 3];
    let i0 = ints[0];
    let opt_first = ints.get(0);
    let opt_fourth = ints.get(3);
    println!("i0 = {}", i0);
    println!("opt_first = {:?}", opt_first);
    println!("opt_fourth = {:?}", opt_fourth);
    println!("opt_first {} {}", opt_first.is_some(), opt_first.is_none());
    println!("opt_fourth {} {}", opt_fourth.is_some(), opt_fourth.is_none());
    println!("opt_first.unwrap => {}", opt_first.unwrap());
    if false { // Avoid panic: called unwrap on None
        println!("opt_fourth.unwrap => {}", opt_fourth.unwrap());
    }
}

#[async_std::main]
async fn main() {
    hellos().await;
    sum_i32();
    sum_f64();
    sqr_it();
    print_factorials();
    use_by_ref();
    use_modifies();
    maths();
    cosine();
    arrays();
    sum_array();
    print_arrays();
    option_arrays();
}
