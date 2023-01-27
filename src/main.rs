use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::net::{IpAddr};
use mpsc::channel;

use crate::icmp::{ping, PingRequest, listen};
use crate::ip_space::{next_ip, NUM_IPS};

mod icmp;
mod ip_space;

fn main() {
    // number of ips to be scanned(default is all NUM_IPS, set to lower for testing)
    let num_ips_to_scan = 100000;
    
    // Create listner thread. Leave unblocking until all requests are sent.
    let listner_thread = thread::spawn(move || {
        listen();
    });

    // create send/receiver vars
    let (tx, _rx) = channel();

    // Start timer
    let start = Instant::now();

    // Initialize algorithm variables
    let mut step:u32 = 0;
    let mut visited = vec![false; NUM_IPS.try_into().unwrap()];
    
    // Send ICMP requests
    for _ in 0..num_ips_to_scan {
        let tx1 = tx.clone();
        
        let target: IpAddr = next_ip(&mut step, &mut visited);

        println!("[{}] {:?}", step, target);

        ping(PingRequest::new(target));
        tx1.send(target.to_string()).unwrap();
    }

    // End Timer for sending
    let duration = start.elapsed();
    
    // Print debug
    println!("Done sending! took {:?} for {:?} targets, or {:?} hours for ipv4", duration, num_ips_to_scan, (((duration / num_ips_to_scan) * 4_294_967_295) / 60) / 60);
    
    // Start blocking main thread for listner
    listner_thread.join().unwrap();

    // for _ in 0..num_ips_to_scan {
    //     println!("{}", rx.recv().unwrap());
    // }

}
