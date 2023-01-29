use std::net::IpAddr;

use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes, IcmpCode, checksum, echo_reply};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{transport_channel, icmp_packet_iter}; 
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

fn ip_to_subnet(ip: &str) -> String {
    let ip_parts: Vec<&str> = ip.split(".").collect();
    let subnet = format!("{}.{}.{}.0/24", ip_parts[0], ip_parts[1], ip_parts[2]);
    subnet
}

// Listener for icmp packets
pub fn listen(filename: String) {
    init_file(&filename);

    let (_, mut tr) = transport_channel(64, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))).unwrap();

    let mut receiver = icmp_packet_iter(&mut tr);

    loop {
        
        // get next packet if we aren't done
        let (packet, addr) = receiver.next().unwrap();
        if packet.get_icmp_type() == IcmpTypes::EchoReply {
            let echo_reply =  echo_reply::EchoReplyPacket::new(packet.packet()).unwrap();
            if echo_reply.get_identifier() == IDN {
                println!("Received from {}", addr);
                write_data(&filename, ip_to_subnet(&addr.to_string()))
            }
        }
        


    }

} 
