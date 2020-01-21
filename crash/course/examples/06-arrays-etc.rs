mod arrays {
    pub fn main() {
        let nums: [u32; 5] = [1, 2, 3, 10, 12];
        println!("nums = {:?}", nums);

        let nums: [u32; 5] = [42; 5];
        println!("nums = {:?}", nums);

        let mut nums: [u32; 5] = [42; 5];
        println!("nums = {:?}", nums);
        nums[2] += 1;
        println!("nums = {:?}", nums);
    }
}
fn main() {
    arrays::main();
}
