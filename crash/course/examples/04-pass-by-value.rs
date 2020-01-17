///
/// Mutable vs immutable pass-by-value
///

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn inc_age_imm(self) -> Self {
        Person {
            name: self.name,
            age: self.age + 1,
        }
    }

    pub fn inc_age_mut(mut self: Self) -> Self {
        self.age += 1;
        self
    }
}

pub fn main() {
    let alice1 = Person {
        name: "Alice".into(),
        age: 30,
    };

    //
    // error[E0594]: cannot assign to `alice1.name`, as `alice1` is not declared as mutable
    // help: consider changing this to be mutable: `mut alice1`
    //
    /*
    alice1.name = "Fred".into();
    */

    //
    // error[E0384]: cannot assign twice to immutable variable `alice1`
    // help: make this binding mutable: `mut alice1`
    //
    /*
    alice1 = Person {
        name: "Fred".into(),
        age: 35,
    };
    */

    println!("alice1 = {:#?}", alice1);
    let alice2 = alice1.inc_age_imm();
    println!("alice2 = {:#?}", alice2);
    let alice3 = alice2.inc_age_mut();
    println!("alice3 = {:#?}", alice3);
}
