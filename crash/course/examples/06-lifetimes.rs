#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn get_older_name<'a>(person1: &'a Person, person2: &'a Person) -> &'a String {
    if person1.age >= person2.age {
        &person1.name
    } else {
        &person2.name
    }
}

fn k(fred: &Person) {
    let wilma = Person {
        name: "Wilma".to_string(),
        age: 35,
    };
    let older_name: &String = get_older_name(&fred, &wilma);
    println!("older_name = {}", older_name);
    println!("fred = {:?}", fred);
}

fn main() {
    let fred = Person {
        name: "Fred".to_string(),
        age: 35,
    };
    k(&fred);
}
