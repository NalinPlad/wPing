use std::io::{stdout};
use std::io::Write;
use std::thread;
use std::time::Instant;
use std::net::{IpAddr};

use crate::icmp::{ping, PingRequest, listen};
use crate::ip_space::{next_ip, NUM_IPS};

mod icmp;
mod ip_space;

fn main() {
    // number of ips to be scanned(default is all NUM_IPS, set to lower for testing)
    const NUM_IPS_TO_SCAN:u32 = 100_000;
    const MAX_THREADS: usize = 1_000;
    // max seconds to wait for packets until exiting
    let _max_timeout = 5;
    
    // Create listner thread. Leave unblocking until all requests are sent.
    let listner_thread = thread::spawn(move || {
        listen(format!("data_{}.csv", NUM_IPS_TO_SCAN).to_string());
    });

    // create send/receiver vars
    // let (tx, _rx) = channel();

    // Start timer
    let start = Instant::now();

    // Initialize algorithm variables
    let mut step:u32 = 0;
    let mut visited = vec![false; NUM_IPS.try_into().unwrap()];
    
    // Create thread pool
    let pool = threadpool::Builder::new()
    .num_threads(MAX_THREADS)
    .build();
        
    
    // Send ICMP requests
    for _ in 0..NUM_IPS_TO_SCAN {
        // let tx1 = tx.clone();
        
        // let gen_ip = Instant::now();
        let target: IpAddr = next_ip(&mut step, &mut visited);
        // println!("next ip took {:?}", gen_ip.elapsed());

        print!("\r[{} / {} {}%]", step, NUM_IPS_TO_SCAN, step as f32 / NUM_IPS_TO_SCAN as f32 * 100.0);
        stdout().flush().unwrap();

        pool.execute(move || {
            ping(PingRequest::new(target));
        });

        // let ping_time = Instant::now();
        // println!("ping took {:?}", ping_time.elapsed())
        // tx1.send(target.to_string()).unwrap();
    }

    // End Timer for sending
    let duration = start.elapsed();
    
    // Print debug
    println!("\nDone sending! took {:?} for {:?} targets, or {:?} hours for ipv4", duration, NUM_IPS_TO_SCAN, (((duration / NUM_IPS_TO_SCAN) * 4_294_967_295) / 60) / 60);
    
    // Start blocking main thread for listner
    listner_thread.join().unwrap();

    // for _ in 0..num_ips_to_scan {
    //     println!("{}", rx.recv().unwrap());
    // }

}
