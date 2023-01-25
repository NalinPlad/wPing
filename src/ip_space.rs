use std::net::{IpAddr, Ipv4Addr};

pub const NUM_IPS: u32 = u32::MAX;

// Durstenfeld shuffle algorithm
pub fn next_ip(step: &mut u32, visited: &mut Vec<bool>) -> IpAddr {
    if step < NUM_IPS {
        
    }
    return IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
}