#[macro_use]
mod macros;

use std::fmt::{self, Display, Formatter};
struct Foo;
impl Display for Foo {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "ignored result")?;

        let mut closure = || writeln!(fmt, "not ignoring result");

        closure()?;

        Ok(())
    }
}
fn check_foo() {
    println!("{}", Foo);
}
fn err_function() -> Result<(), String> {
    Err(String::from("error from function"))
}
fn check_fn() {
    err_function();
}
fn check_ok_macro() {
    ok_macro!();
    match ok_macro!() {
        _ => (),
    }
}
fn check_err_macro() {
    err_macro!();
    match err_macro!() {
        _ => (),
    }
}
fn main() {
    check_foo();
    check_fn();
    check_ok_macro();
    check_err_macro();
    macros::fred();
}
