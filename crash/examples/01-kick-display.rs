use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

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
        [Nested(false), Nested(true), Write]
            .iter()
            .fold(Ok(()), move |acc, ty| {
                // TODO: How to instead capture the `fmt` here?
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
                acc.and(fmt.write_str("\n  "))
                    .and(ty.fmt(fmt))
                    .and(fmt.write_str(": "))
                    .and(_helper(fmt))
                    .and(fmt.write_str("\n")) // TODO: How to intercalate this?
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
