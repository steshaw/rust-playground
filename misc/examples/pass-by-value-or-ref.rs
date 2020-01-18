///
/// Inspiration:
/// https://blog.ryanlevick.com/posts/rust-pass-value-or-reference/
///
/// The key idea is that "Rust is strictly a pass-by-value language".
///

mod inc {
    fn inc(n_inc: usize) -> usize {
        n_inc + 1
    }
    pub fn main() {
        println!("\ninc");
        let n_main: usize = 100;
        println!("{}", inc(n_main));
    }
}

mod usize {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn inc_ref1(n_inc_ref: &usize) -> usize {
        n_inc_ref + 1
    }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn inc_ref2(n_inc_ref: &usize) -> usize {
        *n_inc_ref + 1
    }
    pub fn main() {
        println!("\nusize");
        let n_main: usize = 100;
        let n_main_ref: &usize = &n_main;
        println!("{}", inc_ref1(n_main_ref));
        println!("{}", inc_ref1(&n_main));
        println!("{}", inc_ref2(n_main_ref));
        println!("{}", inc_ref2(&n_main));
    }
}
mod u32 {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn inc_ref1(n_inc_ref: &u32) -> u32 {
        n_inc_ref + 1
    }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn inc_ref2(n_inc_ref: &u32) -> u32 {
        *n_inc_ref + 1
    }
    pub fn main() {
        println!("\nu32");
        let n_main: u32 = 100;
        let n_main_ref: &u32 = &n_main;
        println!("{}", inc_ref1(n_main_ref));
        println!("{}", inc_ref1(&n_main));
        println!("{}", inc_ref2(n_main_ref));
        println!("{}", inc_ref2(&n_main));
    }
}
mod u128 {
    fn inc_ref1(n_inc_ref: &u128) -> u128 {
        n_inc_ref + 1
    }
    fn inc_ref2(n_inc_ref: &u128) -> u128 {
        *n_inc_ref + 1
    }
    pub fn main() {
        println!("\nu128");
        let n_main: u128 = 100;
        let n_main_ref: &u128 = &n_main;
        println!("{}", inc_ref1(n_main_ref));
        println!("{}", inc_ref1(&n_main));
        println!("{}", inc_ref2(n_main_ref));
        println!("{}", inc_ref2(&n_main));
    }
}

mod array {
    fn print_array(array: [Vec<u8>; 3]) {
        for e in array.iter() {
            println!("{:?}", e)
        }
    }
    pub fn main() {
        println!("\narray");
        let array_main: [Vec<u8>; 3] = [vec![1], vec![2, 4], vec![]];
        // The array is copied into print_array here but since Vec isn't
        // Copy, the array is considered "moved" and therefore cannot be
        // accessed again in main.
        // This prevents double-free of Vec heap-allocated memory and access
        // to array_main after free.
        print_array(array_main);

        // Compile error: does not implement the 'Copy' trait.
        //print_array(array_main);
    }
}

mod array_by_ref {
    fn print(array_ref: &[Vec<u8>]) {
        for e in array_ref.iter() {
            println!("{:?}", e)
        }
    }
    pub fn main() {
        println!("\narray_by_ref");
        let array_main: [Vec<u8>; 3] = [vec![1], vec![2, 4], vec![]];
        let array_main_ref: &[Vec<u8>] = &array_main;
        print(array_main_ref);
        print(array_main_ref);
        print(array_main_ref);
    }
}

// Uses code similar to the 'array' module.
mod structure {
    #[derive(Clone, Debug)]
    struct Struct(String);
    impl Drop for Struct {
        fn drop(&mut self) {
            println!("dropping {:?}", self);
        }
    }
    fn print_array<T: std::fmt::Debug>(array: [T; 3]) {
        for e in array.iter() {
            println!("{:?}", e)
        }
    }
    pub fn main() {
        println!("\nstructure");
        let arr: [Struct; 3] = [Struct("1".into()), Struct("2".into()), Struct("3".into())];
        #[allow(unused_variables)]
        let arr_ref = &arr;
        // The array is copied into print_array here but since Vec isn't
        // Copy, the array is considered "moved" and therefore cannot be
        // accessed again in main.
        // This prevents double-free of Vec heap-allocated memory and access
        // to array_main after free.
        println!("before print_array");
        print_array(arr); // arr goes out-of-scope here and Drop is invoked.
        println!("after print_array");

        // Compile error:
        // does not implement the 'Copy' trait.
        //print_array(arr);

        // Compile error:
        // cannot move out of `arr` because it is borrowed
        //print_array(*arr_ref);
    }
}

fn main() {
    inc::main();
    usize::main();
    u32::main();
    u128::main();
    array::main();
    array_by_ref::main();
    structure::main();
}
