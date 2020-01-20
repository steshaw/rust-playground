use std::io::{Error, ErrorKind}; // For sloppy errors.
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = Arc::new(Mutex::new("Fearless".to_string()));
    let mut threads = Vec::new();
    for i in 1..=10 {
        let message = message.clone();
        threads.push(std::thread::spawn(move || {
            let result = message.lock();
            if i == 3 {
                panic!("Poison this at {}", i)
            };
            match result {
                Err(err) => eprintln!("Poisoned: {}", err),
                Ok(mut mutex_guard) => {
                    *mutex_guard += "!";
                    println!("{}", *mutex_guard);
                }
            };
        }));
        sleep(Duration::from_millis(1000));
    }

    // Wait for all the threads. Fail if any threads failed.
    let results = threads
        .into_iter()
        .map(|thread| thread.join())
        .collect::<Vec<_>>();
    if results.iter().any(Result::is_err) {
        let err = Error::new(
            ErrorKind::Other,
            format!("Some of the threads failed to join: {:?}", results),
        );
        return Err(Box::new(err));
    }

    Ok(())
}
