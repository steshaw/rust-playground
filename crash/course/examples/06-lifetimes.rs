#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn get_name(person: &Person) -> &String {
    &person.name
}

fn main() {
    let fred = Person {
        name: "Fred".to_string(),
        age: 35,
    };
    let name: &String = get_name(&fred);
    println!("name = {}", name);
    println!("fred = {:?}", fred);
}
