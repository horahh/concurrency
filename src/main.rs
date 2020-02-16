use std::thread;
use std::sync::{Arc, Mutex};

static FTHREAD: usize = 10;
static STHREAD: usize = 1000;
static VSIZE  : usize = 10;


/****************************************************************************
* Fcuntions to try Rust concurrency
****************************************************************************/
fn main() {
    fibonacci_mt();
    sum_mt();
}

/****************************************************************************
* Spawn threads that calculete the fibonnaci basec on the thread number
* Number of threads match the number of elements in the vector
****************************************************************************/

fn fibbonacci_mt() {
    let mut threads = Vec::new();

    let fib = vec![0;VSIZE];

    //let x = 0;

    // A thread-safe, sharable mutex object
    let data = Arc::new(Mutex::new(fib));

    for i in 0..FTHREAD {
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

}

fn fibonacci( fib : usize ) -> usize {
    let mut result: usize = 0;
    if fib == 0 {
        result=1;
        return result;
    }
    for i in 1..fib {
        result +=i;
    }
    return result;
}

/****************************************************************************
* Spawn threads that sum 1 to a element of a vector
* Sum is based on  the thread number module the vector size
* so that many threads write to the same element.
* 100 threads write to each of the 10 elements of the vector
****************************************************************************/

fn sum_mt() {
    let mut threads = Vec::new();

    let sum = vec![0;VSIZE];

    //let x = 0;

    // A thread-safe, sharable mutex object
    let data = Arc::new(Mutex::new(sum));

    for i in 0..STHREAD {
        // Increment the count of the mutex
        let mutex = data.clone();

        threads.push(thread::spawn(move || {
            // Lock the mutex
            let n = mutex.lock();
            //println!("thread #{}", i);

            let vec_idx = i % VSIZE;

            match n {
                Ok(mut n) => n[vec_idx] = add1(n[vec_idx]),
                Err(str) => println!("{}", str)
            }
        }));
    }

    // Wait all threads ending
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    let new_sum=data.lock().unwrap();

    for i in 0..new_sum.len() {
        println!("sum[{}]={}",i,new_sum[i]);
    }
    
}

fn add1( number : usize ) -> usize {
    let result = number + 1;
    return result ;
}
