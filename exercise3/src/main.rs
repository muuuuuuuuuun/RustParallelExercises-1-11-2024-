//-- #########################
//-- Task: Spawning 10 threads in Rust
//-- #########################

use std::thread;

static NO_THREADS: i32 = 10;

fn main() {
    let mut thread_holder = vec![];

    for i in 0..NO_THREADS {
        thread_holder.push(thread::spawn(move || {
            println!("Thread number is {}", i);
            i // Returning i from the thread
        }));
    }

    println!("***************************");

    for thread_element in thread_holder {
        println!("Thread returned {:?}", thread_element.join().unwrap());
    }
}
