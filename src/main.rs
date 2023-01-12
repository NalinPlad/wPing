use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    // number of threads to be created
    let num_threads = 10000;

    // create send/receiver vars
    // to move data through channel
    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx1.send(i).unwrap();
        });
    }

    for _ in 0..num_threads {
        println!("{}", rx.recv().unwrap());
    }
    println!("Done!");
}
