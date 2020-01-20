use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let message = Arc::new(Mutex::new("Fearless".to_string()));
    for _thread in 1..=10 {
        let message = message.clone();
        std::thread::spawn(move || {
            let result = message.lock();
            match result {
                Err(err) => eprintln!("Poisoned: {}", err),
                Ok(mut mutex_guard) => {
                    *mutex_guard += "!";
                    println!("{}", *mutex_guard);
                }
            };
        });
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
