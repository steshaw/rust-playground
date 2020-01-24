use std::sync::atomic::*;
use std::sync::Arc;
use std::thread::*;
use std::time::Duration;

#[derive(Debug)]
struct Interval {
    counter: Arc<AtomicUsize>,
    running: Arc<AtomicBool>,
}

impl Drop for Interval {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Interval {
    fn from_millis(millis: u64) -> Interval {
        let duration = Duration::from_millis(millis);

        let running = Arc::new(AtomicBool::new(true));
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_t = counter.clone();
        let running_t = running.clone();

        spawn(move || {
            println!("Interval: thread launched!");
            while running_t.load(Ordering::Relaxed) {
                println!("Interval: running = {:?}", running_t);
                sleep(duration);
                let prev_counter = counter_t.fetch_add(1, Ordering::SeqCst);
                println!("prev_counter = {}", prev_counter);
            }
            println!("Interval: thread ending, running = {:?}", running_t);
        });

        Interval { counter, running }
    }

    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

fn main() {
    let interval = Interval::from_millis(250);
    sleep(Duration::from_millis(1000));
    println!("Stopping Interval");
    interval.running.store(false, Ordering::SeqCst);
    println!("Giving some time for Interval to stop");
    sleep(Duration::from_millis(1000));
    println!("Final interval.counter = {}", interval.get_counter());
}
