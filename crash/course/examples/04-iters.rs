mod closure {
    pub fn main() {
        let nums = vec![1, 2, 3, 4, 5];
        (1..3).for_each(|i| nums.iter().for_each(|j| println!("{},{}", i, j)));
    }
}

mod iteration_ref1 {
    pub fn main() {
        println!("\niteration_ref1");
        let nums = vec![1, 2, 3];

        //nums = vec![]; // Compile error! Good.

        for i in 1..3 {
            for j in &nums {
                let _: &u32 = j;
                println!("{},{}", i, j);
            }
        }
    }
}

mod iteration_ref2 {
    pub fn main() {
        println!("\niteration_ref2");
        let nums = &vec![1, 2, 3];
        println!("{:?}", nums); // Use nums to avoid warning.

        //nums = vec![]; // Compile error! Good.

        for i in 1..3 {
            for j in nums {
                let _: &u32 = j;
                println!("{},{}", i, j);
            }
        }
    }
}

mod iteration_mut_ref {
    pub fn main() {
        println!("\niteration_mut_ref");
        let mut nums = vec![1, 2, 3];
        println!("{:?}", nums); // Use nums to avoid warning.

        // Hah! In our example, nums does not need to be assignable.
        // This could be programmed by accident.
        // All we need is to be allowed to take a mutable reference to nums.
        // Must be missing something here...
        nums = vec![];

        for i in 1..3 {
            for j in nums.iter_mut() {
                let _: &mut u32 = j;
                println!("{},{}", i, j);
            }
        }
    }
}

fn main() {
    if false {
        closure::main()
    };
    iteration_ref1::main();
    iteration_ref2::main();
    iteration_mut_ref::main();
}
