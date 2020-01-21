#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn print_person(person: Person) {
    match &person.name {
        Some(name) => {
            println!("Name is {}", name);
        }
        None => {
            println!("No name provided");
        }
    }
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
