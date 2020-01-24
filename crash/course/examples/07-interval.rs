use std::sync::atomic::*;
use std::sync::Arc;
use std::thread::*;
use std::time::Duration;

// #[derive(Clone)]
#[derive(Debug)]
struct Interval {
    counter: Arc<AtomicUsize>,
    //running: AtomicBool,
    running: bool,
}

impl Drop for Interval {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Interval {
    fn from_millis(millis: u64) -> Interval {
        let duration = Duration::from_millis(millis);

        //let running = AtomicBool::new(true);
        let running = true;
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_into_thread = counter.clone();
        spawn(move || {
            println!("Interval thread launched!");

            while running {
                println!("Interval: running = {}", running);
                sleep(duration);
                let prev_counter = counter_into_thread.fetch_add(1, Ordering::SeqCst);
                println!("prev_counter = {}", prev_counter);
            }
        });

        Interval { counter, running }
    }
}

fn main() {
    let mut interval = Interval::from_millis(100);
    sleep(Duration::from_millis(1000));
    println!("Stopping Interval");
    interval.running = false;
    println!("Giving some time for Interval to stop");
    sleep(Duration::from_millis(1000));
}
