// FIXME: the horrors.

use std::iter::Peekable;

#[derive(Debug)]
struct Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    inject: bool,
    t: T,
    iter: Peekable<I>,
}

impl<T, I> Iterator for Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if !self.inject {
            self.inject = !self.inject;
            self.iter.next()
        } else {
            let next = self.iter.peek();
            if next.is_some() {
                self.inject = !self.inject;
                Some(self.t)
            } else {
                self.iter.next() // Iteration ends.
            }
        }
    }
}

fn intersperse<T, I>(x: T, xs: I) -> Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    Intersperse {
        inject: false,
        t: x,
        iter: xs.peekable(),
    }
}

fn intersperse_main() {
    let xs = vec![1, 2, 3];
    let r: Vec<&i32> = intersperse(&42, xs.iter()).collect();
    assert_eq!(vec![&1, &42, &2, &42, &3], r);
    println!("r = {:#?}", r);
}

//
// TODO: Implement intersperse over iterators.
//
// ghci> intersperse 0 [1,2,3]
// [1,0,2,0,3]
//
// TODO: Is this `join` from the std? i.e. `xs.iter().join(0)`?
//
// FIXME: Implement this better :)
// FIXME: Use a type paramater (rather than u32).
// FIXME: Why all the `&` and lifetimes?
//
fn intersperse_vec<'a>(a: &'a u32, xs: Vec<&'a u32>) -> Vec<&'a u32> {
    let mut result: Vec<&'a u32> = vec![];
    let mut iter = xs.iter();
    let mut next = iter.next();
    while next.is_some() {
        result.push(next.unwrap());
        next = iter.next();
        if next.is_some() {
            result.push(a)
        };
    }
    result
}

fn main() {
    let xs: Vec<&u32> = vec![&1, &2, &3];

    let ys = intersperse_vec(&0, xs);
    println!("ys = {:#?}", ys);
    for y in ys.iter() {
        println!("y = {}", y);
    }

    println!("\nintersperse_main:");
    intersperse_main();
}
