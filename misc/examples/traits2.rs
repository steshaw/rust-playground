use std::fmt::Display;

trait Hash {
    fn hash(&self) -> u64;
}

impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self {1} else {0}
    }
}

impl Hash for u64 {
    fn hash(&self) -> u64 {
        *self
    }
}

fn print_hash<A: Hash + Display>(a: A) {
    println!("{}.hash = {}", a, a.hash())
}

fn main() {
    print_hash(true);
    print_hash(42 as u64);
}
