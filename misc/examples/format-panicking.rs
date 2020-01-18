use std::fmt;

struct WriteError;

impl fmt::Display for WriteError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

fn main() {
    let _s: String = format!("Here it is: {}", WriteError);
}
