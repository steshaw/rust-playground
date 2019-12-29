fn main() {
    let v1 = vec![1,2,3];

    v1.iter().for_each(|i| println!("{:?}", i));
    (&v1).iter().for_each(|i| println!("{:?}", i));
    #[allow(clippy::into_iter_on_ref)]
    (&v1).into_iter().for_each(|i| println!("{:?}", i));

    println!("{}", v1.iter().any(|&i| i == 3));
    println!("{}", v1.into_iter().any(|i| i == 3));
}
