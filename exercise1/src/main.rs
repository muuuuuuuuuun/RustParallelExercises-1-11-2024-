//-- #########################
//-- Task: Passing values to a thread in Rust
//-- #########################

use std::thread;

fn main() {
    let x = 1;
    let handle = thread::spawn(move || {
        x
    });

    println!("{:?}", handle.join().unwrap());
}
