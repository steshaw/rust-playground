///
/// Mutable vs immutable pass-by-value
///

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

pub fn inc_age_imm(person: Person) -> Person {
    Person {
        name: person.name,
        age: person.age + 1,
    }
}

pub fn inc_age_mut(mut person: Person) -> Person {
    person.age += 1;
    person
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
    let alice2 = inc_age_imm(alice1);
    println!("alice2 = {:#?}", alice2);
    let alice3 = inc_age_mut(alice2);
    println!("alice3 = {:#?}", alice3);
}
