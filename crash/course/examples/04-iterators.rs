struct Empty;

impl Iterator for Empty {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    for _ in Empty {
        panic!("Argh, an element from Empty!");
    }
    println!("done!");
}
