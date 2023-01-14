use std::thread;
use std::sync::mpsc;

use std::net::{IpAddr, Ipv4Addr};

use crate::icmp::send_echo_request;
mod icmp;

fn main() {
    // number of threads to be created
    let num_threads = 10;

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    for _ in 0..num_threads {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let server: IpAddr = IpAddr::V4(Ipv4Addr::new(80, 9, 12, 3));
            let output: u16 = send_echo_request(server);
            tx1.send(output).unwrap();
        });
    }

    for _ in 0..num_threads {
        println!("{}", rx.recv().unwrap());
    }
    println!("Done!");
}
