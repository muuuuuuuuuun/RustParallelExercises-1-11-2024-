//-- #########################
//-- Task: Spawning multiple threads in Rust
//-- #########################

use std::thread;

fn main() {
    thread::spawn(move || {
        println!("Hello from the first spawned thread");
    });

    let join_handle = thread::spawn(move || {
        println!("Hello from the second spawned thread");
        880088 // Returning a value from this thread
    });

    println!("Hello from the main thread");

    match join_handle.join() {
        Ok(x) => println!("Second spawned thread returned {}", x),
        Err(_) => println!("Second spawned thread panicked"),
    }
}
