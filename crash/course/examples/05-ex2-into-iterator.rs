use std::iter;

struct InfiniteUnit;

/*
impl Iterator for InfiniteUnit {
    type Item = ();
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        Some(())
    }
}
*/

impl IntoIterator for InfiniteUnit {
    type Item = ();
    type IntoIter = iter::Repeat<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        iter::repeat(())
    }
}

fn main() {
    for (count, unit) in InfiniteUnit.into_iter().enumerate() {
        println!("{:?}: count == {}", unit, count);
        if count >= 5 {
            break;
        }
    }
}
