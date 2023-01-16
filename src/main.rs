use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::net::{IpAddr, Ipv4Addr};
use rand::distributions::{Distribution, Uniform};

use crate::icmp::{ping, PingRequest};
mod icmp;

fn main() {
    // number of threads to be created
    let num_threads = 10000;

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    // Start timer
    let start = Instant::now();

    let addr_ranges = Uniform::from(1..=255);

    for _ in 0..num_threads {
        let tx1 = tx.clone();
        //thread::spawn(move || {
            let mut rng = rand::thread_rng();
            
            let target: IpAddr = IpAddr::V4(Ipv4Addr::new(
                    addr_ranges.sample(&mut rng), 
                    addr_ranges.sample(&mut rng), 
                    addr_ranges.sample(&mut rng), 
                    addr_ranges.sample(&mut rng)
                    ));
            let output: u16 = ping(PingRequest::new(target));
            tx1.send(output).unwrap();
        //});
    }

    for _ in 0..num_threads {
        println!("{}", rx.recv().unwrap());
    }

    // End Timer
    let duration = start.elapsed();
    
    println!("Done! took {:?}", duration);
}
