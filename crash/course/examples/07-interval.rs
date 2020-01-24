use std::sync::atomic::*;
use std::sync::*;
use std::thread::*;
use std::time::Duration;

#[derive(Clone, Debug)]
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
    fn from_millis(millis: u64) -> Arc<Interval> {
        let duration = Duration::from_millis(millis);

        let running = Arc::new(AtomicBool::new(true));
        let counter = Arc::new(AtomicUsize::new(0));

        let interval = Arc::new(Interval { counter, running });

        let interval_t = interval.clone();

        spawn(move || {
            println!("Interval: thread launched!");
            while interval_t.running.load(Ordering::SeqCst) {
                println!("Interval: running = {:?}", interval_t);
                sleep(duration);
                let _ = interval_t.counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("Interval: thread ending, interval = {:?}", interval_t);
        });

        interval
    }

    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

fn main() {
    let interval = Interval::from_millis(500);
    let mut last = interval.get_counter();

    for i in 0..50 {
        let curr = interval.get_counter();
        if curr != last {
            last = curr;
            println!("i = {}, counter = {}", i, curr);
        }
        sleep(Duration::from_millis(100));
    }

    println!("Stopping Interval");
    interval.running.store(false, Ordering::SeqCst);
    println!("Giving some time for Interval to stop");
    sleep(Duration::from_millis(1000));
    println!("Final interval.counter = {}", interval.get_counter());
}
