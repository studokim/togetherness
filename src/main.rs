use std::{thread::sleep, time};

fn main() {
    println!("sleeping...");
    let timer = time::Duration::from_secs(100);
    sleep(timer);
    println!("Hello, anyone!");
}
