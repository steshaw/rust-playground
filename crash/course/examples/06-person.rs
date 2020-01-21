#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn print_person(mut person: Person) {
    person.name = match person.name {
        Some(name) => {
            println!("Name is {}", name);
            Some(name)
        }
        None => {
            println!("No name provided");
            None
        }
    };

    match person.age {
        Some(age) => println!("Age is {}", age),
        None => println!("No age provided"),
    }
    println!("person = {:?}", person);
}

fn main() {
    print_person(Person {
        name: Some(String::from("Alice")),
        age: Some(30),
    });
}
