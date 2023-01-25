use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::net::{IpAddr, Ipv4Addr};
use rand::distributions::{Distribution, Uniform};
use mpsc::channel;

use crate::icmp::{ping, PingRequest, listen};
use crate::ip_space::{next_ip, NUM_IPS};

mod icmp;
mod ip_space;

fn main() {
    // number of ips to be scanned(default is all ips, set to lower for testing)
    let num_ips_to_scan = NUM_IPS;

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = channel();

    // Start timer
    let start = Instant::now();

    let addr_ranges = Uniform::from(1..=255);

    for _ in 0..num_ips_to_scan {
        let tx1 = tx.clone();
        //thread::spawn(move || {
            let target: IpAddr = next_ip(step, visited);
            ping(PingRequest::new(target));
            tx1.send(target.to_string()).unwrap();
        //});
    }

    // End Timer
    let duration = start.elapsed();
    
    println!("Done sending! took {:?} for {:?} targets, or {:?} hours for ipv4", duration, num_threads, (((duration / num_threads) * 4_294_967_295) / 60) / 60);
    
    // let (tx_listen, rx_listen) = channel();
    let listner_thread = thread::spawn(move || {
        listen();
    });

    listner_thread.join().unwrap();

    for _ in 0..num_threads {
        println!("{}", rx.recv().unwrap());
    }

}
