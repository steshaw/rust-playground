/*
// XXX: ARGH
use std::slice::Iter;

struct Intersperse<T> {
    t: T,
}
use std::iter::*;

fn intersperse<'a, T>(inject: Iter<T>, xs: Iter<T>)
//std::iter::FlatMap<std::slice::Iter<'_, T>, std::iter::Chain<std::iter::Once<&T>, std::slice::Iter<'_, T>>
{
    fn f(x: &u8) {
        [x].iter().chain(inject)
    }
    xs.flat_map(f);
}
*/

//#[derive(Debug)]
struct Intersperse<I>
where I: IntoIterator,
      I: Iterator
{
    first : bool,
    t: <I as Iterator>::Item,
    iter: I,
}

impl<I> Iterator for Intersperse<I>
where I : Iterator
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(_x) = self.iter.next() {
//            Some(self.t)
            Some(_x)
        } else { None }
    }
}

fn intersperse<I>(x: <I as Iterator>::Item, xs: I) -> Intersperse<I>
where
    I: IntoIterator,
    I: Iterator,
{
    Intersperse { first : false, t: x, iter: xs }
}

fn intersperse_main() {
    let xs = vec![1,2,3];
    let r : Vec<&i32> = intersperse(&0, xs.iter()).collect();
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
