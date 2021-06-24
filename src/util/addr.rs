use std::net::{IpAddr, Ipv6Addr};

pub fn ip_to_bytes<'a>(ip: IpAddr) -> [u8; 16] {
    let ipv6_addr = match ip {
        IpAddr::V4(ip) => ip.to_ipv6_mapped(),
        IpAddr::V6(ip) => ip,
    };

    ipv6_addr.octets()
}

pub fn bytes_to_ip(bytes: &[u8]) -> Option<IpAddr> {
    if bytes.len() != 16 {
        return None;
    }

    let mut s_bytes: [u8; 16] = [0; 16];
    for i in 0..16 {
        s_bytes[i] = bytes[i];
    }

    let ip = Ipv6Addr::from(s_bytes);
    let ip = match Ipv6Addr::from(s_bytes).to_ipv4() {
        Some(ip) => IpAddr::V4(ip),
        None => IpAddr::V6(ip),
    };

    Some(ip)
}
