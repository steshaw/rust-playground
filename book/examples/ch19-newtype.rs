use std::fmt;

struct NixList(Vec<String>);

impl fmt::Display for NixList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\n  {}\n]", self.0.join("\n  "))
    }
}

fn main() {
    let v = vec![String::from("hello"), String::from("world")];
    let w = NixList(vec![String::from("hello"), String::from("world")]);
    println!("v = {:?}", v);
    println!("w = {}", w);
}
