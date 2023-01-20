use std::net::IpAddr;

use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes, IcmpCode, checksum};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::transport_channel; 
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
use rand::{Rng};

pub struct PingRequest {
    addr: IpAddr,
    identifier: u16
}


impl PingRequest {
    pub fn new(addr: IpAddr) -> PingRequest {
        let mut rng = rand::thread_rng();
        PingRequest {
            addr,
            identifier: rng.gen::<u16>(),
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
    tx.send_to(packet, dest.get_addr()).unwrap();  
}

// Listener for icmp packets
pub fn listen() {
    let (_, mut tr) = transport_channel(64, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))).unwrap();
    
    println!("Hellotdst {:?}", tr.buffer)
    // loop {
    //     match tr.next() {
    //         Ok(packet) => {
    //             println!("Received ICMP Echo Reply Packet: {:?}", packet);
    //         },
    //         Err(e) => {
    //             println!("Error while receiving packet: {:?}", e);
    //         }
    //     }
    // }
} 
