use std::thread;
use std::sync::mpsc;

use std::time::{Duration, Instant};

use std::net::{IpAddr, Ipv4Addr};

use crate::icmp::{ping, PingRequest};
mod icmp;

fn main() {
    // number of threads to be created
    let num_threads = 10;

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    // Start timer
    let start = Instant::now();

    for _ in 0..num_threads {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let output: u16 = ping(PingRequest::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
            tx1.send(output).unwrap();
        });
    }

    for _ in 0..num_threads {
        println!("{}", rx.recv().unwrap());
    }

    // End Timer
    let duration = start.elapsed();
    
    println!("Done! took {:?}", duration);
}
