//
// FIXME: Fix the horrors here.
//
// TODO: Implement intersperse over iterators.
//
// ghci> intersperse 0 [1,2,3]
// [1,0,2,0,3]
//
// TODO: Is this `join` from the std? i.e. `xs.iter().join(0)`?
// TODO: Seems it might be available with +nightly.
//

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

//
// Intersperse as an "extension method",
// with help from Jake Goulding (Shepmaster):
//
//   https://stackoverflow.com/a/30540952/482382
//
trait IntersperseExt {
    fn intersperse<T>(self, x : T) -> Intersperse<T, Self>
    where
        Self: Iterator<Item = T>,
        T: Copy,
        Self: Sized
    {
        Intersperse {
            inject: false,
            t: x,
            iter: self.peekable(),
        }
    }
}

impl<I: Iterator> IntersperseExt for I {}

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
    println!("intersperse_vec:");
    let xs: Vec<&u32> = vec![&1, &2, &3];
    let ys = intersperse_vec(&0, xs);
    let zs = ys.iter().copied().copied().collect::<Vec<u32>>();
    assert_eq!(vec![1, 0, 2, 0, 3], zs);
    println!("ys = {:#?}", ys);

    println!("intersperse:");
    let xs = vec![1, 2, 3];
    let ys : Vec<i32> = xs.iter().intersperse(&0).copied().collect();
    assert_eq!(vec![1, 0, 2, 0, 3], ys);
    println!("ys = {:#?}", ys);
}
