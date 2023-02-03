use std::{fs::OpenOptions, io::Write};
use csv::Reader;

pub fn init_file(file_name: &str) {
    let mut _file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    // let header = "block,count\n";
    // file.write_all(header.as_bytes()).unwrap();
}

fn block_to_number(ip_block: &str) -> u32 {
    let parts: Vec<&str> = ip_block.split("/").collect();
    let ip = parts[0];
    let mask = parts[1].parse::<u32>().unwrap();

    let ip_parts: Vec<&str> = ip.split(".").collect();
    let ip_nums: [u32; 4] = [
        ip_parts[0].parse::<u32>().unwrap(),
        ip_parts[1].parse::<u32>().unwrap(),
        ip_parts[2].parse::<u32>().unwrap(),
        ip_parts[3].parse::<u32>().unwrap(),
    ];

    let mut number: u32 = 0;
    for i in 0..4 {
        number += ip_nums[i] * (256u32.pow(3 - i as u32));
    }

    number >> (32 - mask)
}

pub fn write_data(file_name: String, block: String) {
    // Open the file in write mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_name)
        .unwrap();
    
    // Write block
    let b_data = block_to_number(&block).to_string() + "\n"; 
    file.write_all(b_data.as_bytes()).unwrap();
}