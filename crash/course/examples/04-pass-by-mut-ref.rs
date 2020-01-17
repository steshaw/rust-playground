#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn inc_age_imm(person: &mut Person) {
    person.age += 1
}

fn inc_age_mut<'a>(mut person: &'a mut Person, replacement: &'a mut Person) {
    person = replacement;
    person.age += 1
}

fn dump(prefix: &str, alice: &Person, bob: &Person) {
    println!("{}: alice = {:#?}", prefix, alice);
    println!("{}: bob = {:#?}", prefix, bob);
    println!();
}

fn main() {
    let mut alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    let mut bob = Person {
        name: "Bob".into(),
        age: 20,
    };
    dump("1", &alice, &bob);

    inc_age_imm(&mut alice);
    dump("2", &alice, &bob);

    inc_age_mut(&mut alice, &mut bob);
    dump("3", &alice, &bob);
}
