// Fearless concurrency — following along with the article.

use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let msg0 = Arc::new(Mutex::new("Fearless".to_string()));
    for _ in 1..11 {
        let msg1 = msg0.clone();
        let inner = || {
            let msg2 = msg1; // Allows removing `move` on closure.
            let mut msg3 = msg2.lock().unwrap();
            msg3.push('!');
            println!("{}", msg3);
        };
        spawn(inner);
        sleep(Duration::from_millis(100));
    }
}

