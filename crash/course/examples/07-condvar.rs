// Example from https://doc.rust-lang.org/std/sync/struct.Condvar.html?search=#method.wait

use std::sync::*;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(500));
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("loop: started = {}", started);
        started = cvar.wait(started).unwrap();
    }
    println!("end: started = {}", started);
}
