#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn inc_age_imm(person: &Person) -> Person {
    Person {
        name: person.name.clone(),
        age: person.age + 1,
    }
}

#[allow(unused_mut)]
fn inc_age_mut(mut person: &Person) -> Person {
    Person {
        name: person.name.clone(),
        age: person.age + 1,
    }
}

fn dump(prefix: &str, alice: &Person) {
    println!("{}: alice = {:#?}", prefix, alice);
    println!();
}

fn main() {
    let mut alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    dump("1", &alice);

    alice = inc_age_imm(&alice);
    dump("2", &alice);

    alice = inc_age_mut(&alice);
    dump("3", &alice);
}
