use std::thread;
use std::sync::{Arc, Mutex};

static NTHREAD: usize = 10;
static VSIZE  : usize = 10;


fn main() {
    let mut threads = Vec::new();

    let fib = vec![0;VSIZE];

    //let x = 0;

    // A thread-safe, sharable mutex object
    let data = Arc::new(Mutex::new(fib));

    for i in 0..NTHREAD {
        // Increment the count of the mutex
        let mutex = data.clone();

        threads.push(thread::spawn(move || {
            // Lock the mutex
            let n = mutex.lock();
            println!("thread #{}", i);

            match n {
                Ok(mut n) => n[i] = fibonacci(i),
                Err(str) => println!("{}", str)
            }
        }));
    }

    // Wait all threads ending
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    let new_fib=data.lock().unwrap();

    for i in 0..new_fib.len() {
        println!("fibonacci[{}]={}",i,new_fib[i]);
    }

    //assert_eq!(*data.lock().unwrap(), 55);
}

fn fibonacci( fib : usize ) -> usize {
    println!("");
    let mut result: usize = 0;
    if fib == 0 {
        result=1;
        return result;
    }
    for i in 1..fib {
        result +=i;
    }
    return result
}
