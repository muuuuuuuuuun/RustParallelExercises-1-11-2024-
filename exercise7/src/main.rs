//-- #########################
//-- Task: Waiting for a child process
//-- #########################

use std::process::Command;

//replace sleep with timeout/ping

fn main() {
    let mut child = Command::new("ping").arg("127.0.0.1").arg("-n").arg("6").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("Status of child process: {:?}", _result);
    println!("Reached the end of main");
}