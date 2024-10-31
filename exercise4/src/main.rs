//-- #########################
//-- Task: Use channel to perform safe pass of data between threads
//-- #########################

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NO_THREADS: i32 = 3;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for thread_no in 0..NO_THREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(thread_no).unwrap();
            println!("thread {} finished", thread_no);
        });
    }

    let mut thread_holder = Vec::with_capacity(NO_THREADS as usize);

    for _ in 0..NO_THREADS {
        thread_holder.push(rx.recv().unwrap());
    }

    println!("{:?}", thread_holder);
}
