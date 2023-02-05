use core::time;
use std::io::{stdout};
use std::io::Write;
use std::{thread, env};
use std::time::Instant;

use std::net::{IpAddr};


use crate::icmp::{ping, PingRequest, listen};
use crate::ip_space::{next_ip, NUM_IPS};

mod icmp;
mod ip_space;

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(args.len());
    
    // number of ips to be scanned(default is all NUM_IPS, set to lower for testing)
    let NUM_IPS_TO_SCAN:u32 = if !(args.len() > 1) {NUM_IPS} else {args[1].parse::<u32>().unwrap()};
    // let NUM_IPS_TO_SCAN: u32 = 100;

    // max threads for sending, should use double
    const MAX_THREADS: usize = 100;

    // max seconds to wait for packets until exiting
    let _max_timeout = 5;
    
    // Create listner thread. Leave unblocking until all requests are sent.
    let listner_thread = thread::spawn(move || {
        let mut count_recv = 0;
        let device = if !(args.len() > 2) {"en0"} else {&args[2]};
        listen(format!("data_{}.csv", NUM_IPS_TO_SCAN).to_string(), &mut count_recv, device);
    });

    // We need to sleep to give listner time to intialize, or else we loose ~ 5k packets at start
    thread::sleep(time::Duration::from_millis(3000));

    // Start timer
    let start = Instant::now();

    // Initialize algorithm variables
    let mut step:u32 = 0;
    let mut visited = vec![false; NUM_IPS.try_into().unwrap()];
    
    // Create thread pool
    let pool = threadpool::Builder::new()
    .num_threads(MAX_THREADS)
    .build();
        
    // Open a channel to send the packet
    // let (mut tx, _) = transport_channel(64, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))).unwrap();
    
    // Send ICMP requests
    for _ in 0..NUM_IPS_TO_SCAN {
        let target: IpAddr = next_ip(&mut step, &mut visited);
        let t_string = target.to_string();

        if target.is_loopback() 
        || target.is_multicast()
        || t_string.to_string().starts_with("0")
        || t_string.to_string().starts_with("127")
        || t_string.to_string().starts_with("10")
        || t_string.to_string().split(".").collect::<Vec<&str>>()[0].parse::<u8>().unwrap() > 223{
            continue;
        }

        print!("\r[{} / {} {}%", step, NUM_IPS_TO_SCAN, step as f32 / NUM_IPS_TO_SCAN as f32 * 100.0);
        stdout().flush().unwrap();

        pool.execute(move || {
            ping(PingRequest::new(target));
        });
    }

    // End Timer for sending
    let duration = start.elapsed();
    
    // Print debug
    println!("\nDone sending! took {:?} for {:?} targets, or {:?} hours for ipv4", duration, NUM_IPS_TO_SCAN, (((duration / NUM_IPS_TO_SCAN) * 4_294_967_295) / 60) / 60);
    
    // Start blocking main thread for listner
    listner_thread.join().unwrap();

}
