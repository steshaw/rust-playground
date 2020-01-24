use std::sync::atomic::*;
use std::sync::*;
use std::thread::*;

#[derive(Debug)]
struct Producer {
    ready: Arc<Mutex<bool>>,
    ready_cv: Arc<Condvar>,
    counter: Arc<AtomicUsize>,
    running: Arc<AtomicBool>,
    join_handle: JoinHandle<()>,
}

/*
impl Drop for Producer {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
*/

impl Producer {
    fn new() -> Producer {
        let ready = Arc::new(Mutex::new(false));
        let ready_cv = Arc::new(Condvar::new());
        let running = Arc::new(AtomicBool::new(true));
        let counter = Arc::new(AtomicUsize::new(0));

        let ready_t = ready.clone();
        let ready_cv_t = ready_cv.clone();
        let running_t = running.clone();
        let counter_t = counter.clone();

        let join_handle = spawn(move || {
            println!("producer: thread launched!");
            while running_t.load(Ordering::SeqCst) {
                println!("producer: still running, counter = {:?}", counter_t);
                let mut ready = ready_t.lock().unwrap();
                while *ready {
                    println!("producer: before cv.wait ready = {}", ready);
                    ready = ready_cv_t.wait(ready).unwrap();
                    println!("producer: after cv.wait ready = {}", ready);
                }
                // Bump counter (aka produce value).
                let _ = counter_t.fetch_add(1, Ordering::SeqCst);
                println!(
                    "producer: Sending {:?}, setting ready=true, cv.notify_one",
                    counter_t
                );
                *ready = true;
                ready_cv_t.notify_one();
            }
            println!("producer: thread ending, counter = {:?}", counter_t);
        });

        Producer {
            ready,
            ready_cv,
            counter,
            running,
            join_handle: join_handle,
        }
    }

    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

fn main() -> Result<()> {
    // Start producer thread.
    let producer = Producer::new();

    // Grab 10 values from the producer thread.
    let ready = &producer.ready;
    let ready_cv = &producer.ready_cv;
    for i in 0..10 {
        println!("consumer: running i = {}", i);
        let mut ready = ready.lock().unwrap();
        while !*ready {
            println!("consumer: before cv.wait ready = {}", ready);
            ready = ready_cv.wait(ready).unwrap();
            println!("consumer: after cv.wait ready = {}", ready);
        }
        println!(
            "consumer: received counter = {} at i = {}",
            producer.get_counter(),
            i
        );
        println!("consumer: setting ready=false, cv.notify_one");
        *ready = false;
        ready_cv.notify_one();
    }

    println!("Final producer.counter = {}", producer.get_counter());
    println!("Stopping producer");
    producer.stop();
    producer.join_handle.join()?;

    Ok(())
}
