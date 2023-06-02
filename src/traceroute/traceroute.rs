use std::net::IpAddr;
use crate::traceroute::ping::ping;
use dns_lookup::lookup_host;

pub fn trace(addr: String) {
    match get_ip(addr) {
        None => panic!("Failed to parse into a valid ip address"),
        Some(ip) => {
            println!("{:?}", ip);
            ping(ip)
        }
    }
}

fn get_ip(addr: String) -> Option<IpAddr> {
    match addr.parse::<IpAddr>() {
        Ok(ip) => Some(ip),
        Err(_) => {
            get_ip_from_url(addr)
        }
    }
}

fn get_ip_from_url(url: String) -> Option<IpAddr> {
    return match lookup_host(&url) {
        Ok(ips) => {
            let ip = ips.get(0).cloned();
            return ip
        },
        Err(_) => None
    }
}