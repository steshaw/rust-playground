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
fn intersperse<'a>(a: &'a u32, xs: Vec<&'a u32>) -> Vec<&'a u32> {
    let mut result: Vec<&'a u32> = vec![];
    let mut iter = xs.iter();
    let mut x = iter.next();
    while x.is_some() {
        result.push(x.unwrap());
        x = iter.next();
        if x.is_some() {
            result.push(a)
        };
    }
    result
}

fn main() {
    let xs: Vec<&u32> = vec![&1, &2, &3];

    let ys = intersperse(&0, xs);
    println!("ys = {:#?}", ys);
    for y in ys.iter() {
        println!("y = {}", y);
    }
}
