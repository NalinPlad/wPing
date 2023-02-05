use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use pcap::{Capture};

use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes, IcmpCode, checksum};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{transport_channel}; 
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;

use save_data::{init_file, write_data};
mod save_data;

// ICMP identifier for program
const IDN: u16 = 1337;

pub struct PingRequest {
    addr: IpAddr,
    identifier: u16
}


impl PingRequest {
    pub fn new(addr: IpAddr) -> PingRequest {
        PingRequest {
            addr,
            identifier: IDN,
        }
    }

    pub fn get_identifier(&self) -> u16 {
        return self.identifier;
    }
    
    pub fn get_addr(&self) -> IpAddr {
        return self.addr;
    }

}

pub fn ping(dest: PingRequest) {
    // Buffer for packet
    let mut packet_buffer = vec![0u8; 64];

    // Create new packet using the packet buffer
    let mut packet = MutableEchoRequestPacket::new(&mut packet_buffer[..]).unwrap();

    // Add packet headers
    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_sequence_number(0); // 0 for first in sequence
    packet.set_identifier(dest.get_identifier());
    packet.set_payload("".as_bytes());
    packet.set_icmp_code(IcmpCode::new(0));
    
    let checksum = checksum(&IcmpPacket::new(packet.packet()).unwrap());
    packet.set_checksum(checksum);

    // Open a channel to send the packet
    let (mut tx, _) = transport_channel(64, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))).unwrap();

    // Send the packet
    // match tx.send_to(packet, dest.get_addr()) {
    //     Ok(_) => {},
    //     Err(_) => {}
    // }
    if let Ok(_) = tx.send_to(packet, dest.get_addr()) {
    } else { // cant ping ip, so dont do anything
    }        // have to do this because it panics otherwise
}

fn ip_to_subnet(ip: String) -> String {
    let ip_parts: Vec<&str> = ip.split(".").collect();
    let subnet = format!("{}.{}.{}.0/24", ip_parts[0], ip_parts[1], ip_parts[2]);
    subnet
}

// Listener for icmp packets
pub fn listen(filename: String, count_recv: &mut u32, device: &str) {
    init_file(&filename);

    // let (_, mut tr) = transport_channel(64, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))).unwrap();

    // let mut receiver = icmp_packet_iter(&mut tr);
    
    let mut cap = Capture::from_device(device) // open the "default" interface
              .unwrap() // assume the device exists and we are authorized to open it
              .timeout(5000)
              .buffer_size(64*1000) // block for 1 minute just in case 
              .open() // activate the handle
              .unwrap(); // assume activation worked

    // Filter by our ICMP packets
    cap.filter(r"icmp", false).unwrap();
    
    // Create thread pool
    let pool = threadpool::Builder::new()
    .num_threads(100)
    .build();

    // Get packets
    while let Ok(packet) = cap.next_packet() {
        // get next packet if we aren't done
        // let (packet, addr) = receiver.next().unwrap();
        let data = packet.data;
        if data[34] != 0 {
            continue;
        }

        // https://www.rfc-editor.org/rfc/rfc792.html
        let addr = IpAddr::V4(Ipv4Addr::new(packet.data[26],packet.data[27],packet.data[28],packet.data[29]));
        *count_recv += 1;

        // println!("{} from {:?}, 34 {}", addr, data, data[34]);
        
        let filename = Arc::new(filename.to_string());

        pool.execute(move || {
            let filename = filename.clone();
            write_data(filename.to_string(), ip_to_subnet(addr.to_string()))
        });
    }
    println!("done")

} 
