fn main() {
    let nums: Vec<u32> = (1..11).collect();

    for _ in 1..3 {
        for i in nums.iter().map(|n| n * 2) {
            println!("{}", i);
        }
    }
}
