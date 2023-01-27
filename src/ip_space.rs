use std::net::{IpAddr, Ipv4Addr};

pub const NUM_IPS: u32 = u32::MAX;

pub fn next_ip(step: &mut u32, visited: &mut Vec<bool>) -> IpAddr {
    if step < &mut NUM_IPS {
        let r = fastrand::usize(..visited.len());
        let mut k = 0;
        if !visited[r] {
            visited[r] = true;
        } else {
            while visited[r-k] {
                k += 1;
                if r-k <= 0 {
                    k = r - NUM_IPS as usize; // r - k = n; -k = n - r; k = -n + r; k = r - n
                }
            }        
        }
        step = ;

        return IpAddr::V4(Ipv4Addr::new(
        ((r-k >> 24) & 255).try_into().unwrap(), 
        ((r-k >> 16) & 255).try_into().unwrap(),
        ((r-k >> 8) & 255).try_into().unwrap(),
        (r-k & 255).try_into().unwrap()));
    }
    return IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
}