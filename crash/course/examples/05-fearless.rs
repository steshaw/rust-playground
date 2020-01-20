use std::io::{Error, ErrorKind}; // For sloppy errors.
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use rand;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if false {
        for _ in 0..3 {
            let mut rng = rand::thread_rng();
            let n: u8 = rng.gen_range(0, 10);
            println!("n = {}", n);
        }
        return Ok(());
    }

    let message = Arc::new(Mutex::new("Fearless".to_string()));
    let mut threads = Vec::new();
    for i in 1..=10 {
        let message = message.clone();
        threads.push(std::thread::spawn(move || {
            match message.lock() {
                Err(err) => eprintln!("Poisoned: {}", err),
                Ok(mut mutex_guard) => {
                    // Randomly fail 10% of the time.
                    let n: u8 = rand::thread_rng().gen_range(0, 10);
                    if n == 0 {
                        panic!("Thread {} has poisoned mutex", i)
                    };

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
