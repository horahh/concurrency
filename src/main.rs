use std::thread;
use std::sync::{Arc, Mutex};

static NTHREAD: usize = 10;

fn main() {
    let mut threads = Vec::new();

    let x = 0;

    // A thread-safe, sharable mutex object
    let data = Arc::new(Mutex::new(x));

    for i in 1..(NTHREAD+1) {
        // Increment the count of the mutex
        let mutex = data.clone();

        threads.push(thread::spawn(move || {
            // Lock the mutex
            let n = mutex.lock();
            println!("thread #{}", i);

            match n {
                Ok(mut n) => *n += i,
                Err(str) => println!("{}", str)
            }
        }));
    }

    // Wait all threads ending
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    assert_eq!(*data.lock().unwrap(), 55);
}
