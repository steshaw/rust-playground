fn main() {
    let mut i = 1;

    loop {
        if i > 10 {
            break
        } else {
            println!("i = {}", i);
            i += 1
        }
    }

    for i in 1..=10 {
        println!("i = {}", i)
    }
}
