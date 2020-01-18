use std::fmt;
use no_panic::no_panic;

struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

struct FormatNoPanic;

impl fmt::Display for FormatNoPanic {
    #[no_panic]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}

fn main() {
    println!("Here it is: {}", FormatNoPanic);
    println!("Here it is: {}", FormatError);
}
