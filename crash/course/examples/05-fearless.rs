use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let message = Arc::new(Mutex::new("Fearless".to_string()));
    for _thread in 1..=10 {
        let message = message.clone();
        std::thread::spawn(move || {
            let result = message.lock();
            let mut mutex_guard = result.unwrap();
            *mutex_guard += "!";
            println!("{}", *mutex_guard);
        });
    }
}
