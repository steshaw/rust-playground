use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// TODO: Implement intercalate.
/*
use std::iter::Iterator;

struct Intercalate<T> {}

fn intercalate<T>(xs: &dyn Iterator<T>, ys: &dyn Iterator<T>) -> Intercalate<T> {
    xs.fold(xs, |acc, x| acc.chain(x))
}
fn hello_intercalate() {
    let xs = vec![1, 2, 3];
    let bs = intercalate([0].iter(), xs.iter());
}
*/

impl Display for Person {
    fn fmt(self: &Self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        #[derive(Debug)]
        enum FormatType {
            Nested(bool),
            Write,
        }

        use std::fmt::Debug;
        use FormatType::*;

        // Use all techniques to display on self.
        let mut first = true;
        [Nested(false), Nested(true), Write]
            .iter()
            .fold(Ok(()), move |acc, ty| {
                // TODO: How remove the `fmt` parameter (and capture `fmt` instead)?
                // NOTE; The `fmt` closure argument requires the type annotation.
                let _helper = |fmt: &mut Formatter| match ty {
                    Nested(true) => fmt.write_str(&self.name).and(
                        fmt.write_str(", ").and(
                            fmt.write_str(&self.age.to_string())
                                .and(fmt.write_str(" years old")),
                        ),
                    ),

                    Nested(false) => fmt
                        .write_str(&self.name)
                        .and(fmt.write_str(" bar"))
                        .and(fmt.write_str(", "))
                        .and(fmt.write_str(&self.age.to_string()))
                        .and(fmt.write_str(" years old")),

                    Write => write!(fmt, "{}, {} years of age", self.name, self.age),
                };
                acc.and(if first {
                    first = false;
                    fmt.write_str("\n  ")
                } else {
                    fmt.write_str("\n| ")
                })
                .and(ty.fmt(fmt))
                .and(fmt.write_str(": "))
                .and(_helper(fmt))
            })
    }
}

fn main() {
    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    println!("Display Alice = {}", alice);
    println!("Debug Alice = {:?}", alice);
    println!("Pretty Alice = {:#?}", alice);
}
