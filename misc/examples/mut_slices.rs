use std::fmt;

struct Slice<T> {
    data: *const T,
    len: usize,
}

//impl<T: fmt::Display> Slice<T> {
impl<T> fmt::Display for Slice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slice({:#?}, {})", self.data, self.len) // ; <-- XXX: No semicolon here.
    }
}

fn mutate_vec(v: &mut Vec<i32>) -> () {
    v.push(4);
}

fn mutable_example() {
    println!("mutable_example");
    let v = &mut vec![1, 2, 3];
    let r = mutate_vec(v);

    println!("r is: {:?}", r);
    println!("v is: {:?}", v);
}

fn slice_example() {
    println!("slice_example");
    let v: Vec<u8> = vec![1, 2, 3];
    println!("v is: {:?}", v);
    let slice0 = &v[0..];
    println!("slice0 is: {:?}", slice0);
    let slice1 = &v[1..];
    println!("slice1 is: {:?}", slice1);
    let slice2 = &v[2..];
    println!("slice2 is: {:?}", slice2);
    let slice3 = &v[3..];
    println!("slice3 is: {:?}", slice3);
    if false {
      let slice4 = &v[4..]; // XXX: 'slice index starts at 4 but ends at 3'
      println!("slice4 is: {:?}", slice4);
    }

    let p = &v;
    let our_slice1 = Slice{data : p, len : 1};

    println!("our_slice1.data = {:?}", our_slice1.data);
    println!("our_slice1.len = {}", our_slice1.len);
    println!("our_slice1 = {}", our_slice1);
}

fn main() {
    mutable_example();
    slice_example();
}
