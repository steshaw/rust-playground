fn main() {
    let v1 = vec![1,2,3];

    println!("has any 2's {}", v1.iter().any(|&x| x == 2));
}
