fn main() {
    let mut count = 0;

    (1..=5).for_each(|_| {
        count += 1;
        println!("You are visitor #{}", count);
    })
}

