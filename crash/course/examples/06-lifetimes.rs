#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn get_older_name<'a, 'b: 'a>(person1: &'a Person, person2: &'b Person) -> &'a String {
    if person1.age >= person2.age {
        &person1.name
    } else {
        &person2.name
    }
}

fn main() {
    let fred = Person {
        name: "Fred".to_string(),
        age: 35,
    };
    let wilma = Person {
        name: "Wilma".to_string(),
        age: 35,
    };
    let older_name: &String = get_older_name(&fred, &wilma);
    println!("older_name = {}", older_name);
    println!("fred = {:?}", fred);
}
