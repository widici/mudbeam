use std::net::IpAddr;
use dns_lookup::{lookup_addr, lookup_host};
use crate::error::Error;

pub fn get_ip_addr(addr: String) -> Result<IpAddr, Error> {
    if let Ok(ip) = addr.parse::<IpAddr>() {
        return Ok(ip)
    }

    return get_ip_from_hostname(addr)
}

pub fn get_ip_from_hostname(addr: String) -> Result<IpAddr, Error> {
    let error: Error = Error::new(String::from(format!("Failed to resolve ip address: {}", addr)));
    return match lookup_host(&addr) {
        Ok(ips) => {
            if let Some(ip) = ips.get(0).cloned() {
                return Ok(ip)
            }
            Err(error)
        },
        Err(_) => Err(error)
    }
}

pub fn get_hostname_from_ip(addr: &IpAddr) -> Option<String> {
    return match lookup_addr(addr) {
        Ok(url) => Some(url),
        Err(_) => return None,
    };
}