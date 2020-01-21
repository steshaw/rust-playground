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
mod vec {
    pub fn main() {
        let mut v: Vec<u32> = vec![1, 2, 3];
        println!("v = {:?}", v);
        v.push(4);
        println!("v = {:?}", v);
        v.push(4);
        v.push(5);
        v.push(6);
        println!("v = {:?}", v);
        assert_eq!(v.pop(), Some(6));
        assert_eq!(v[2], 3); // NOTE: Indexing can panic!
        assert_eq!(v.get(2), Some(&3));
        assert_eq!(v.len(), 6);
        v.truncate(2);
        println!("v = {:?}", v);
        assert_eq!(v, vec![1, 2]);
        v.extend([42; 3].iter());
        println!("v = {:?}", v);
        assert_eq!(v, vec![1, 2, 42, 42, 42]);
        v.append(&mut vec![99, 100]);
        println!("v = {:?}", v);
        assert_eq!(v, vec![1, 2, 42, 42, 42, 99, 100]);

        // Drain all elements.
        for n in v.drain(0..) {
            println!("n = {}", n);
        }
        println!("v = {:?}", v);
        assert_eq!(v, Vec::new());
    }
}
mod slices {
    pub fn print_vals(v: &[u8]) {
        for (i, n) in v.iter().enumerate() {
            println!("v[{}] = {}", i, n);
        }
    }
    pub fn main() {
        let mut v: Vec<u8> = vec![1, 2, 3];
        println!("v = {:?}", v);
        assert_eq!(v, vec![1, 2, 3]);
        print_vals(&v);

        v.push(4);
        println!("v = {:?}", v);
        assert_eq!(v, vec![1, 2, 3, 4]);
        print_vals(&v);

        let a: [u8; 5] = [1, 2, 3, 4, 5];
        print_vals(&a);
    }
}
mod deref {
    pub fn main() {
        let v = vec![1, 2, 3];
        let _: &Vec<u32> = &v;
        let _: &[u32] = &v;
    }
}
mod my_array{
    use std::ops::Deref;

    struct MyArray([u8; 5]);

    impl MyArray {
        fn new() -> MyArray {
            MyArray([42; 5])
        }
    }

    impl Deref for MyArray {
        type Target = [u8];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn main() {
        let ma = MyArray::new();
        let _: &MyArray = &ma;
        let _: &[u8] = &ma;
        super::slices::print_vals(&ma);
    }
}
fn main() {
    arrays::main();
    vec::main();
    slices::main();
    deref::main();
    my_array::main();
}
