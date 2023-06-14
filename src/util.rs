use std::net::IpAddr;
use crate::dns::get_hostname_from_ip;
use crate::ping::PingResult;

pub fn summarize_responses(ttl: u8, responses: Vec<PingResult>, target_ip: IpAddr) -> bool {
    let mut ip_addr: Option<IpAddr> = None;
    print!("{}  ", space_format(2, ttl.to_string()));
    for response in responses {
        match response {
            PingResult::Ok { ip, rtt } => {
                print!("{} ms  ", space_format(4, rtt.to_string()));
                if ip_addr.is_none() {
                    ip_addr = Some(ip)
                }
            }
            PingResult::Timeout => print!("    *    ")
        }
    }

    match ip_addr {
        None => print!("Timed out.\n"),
        Some(ip) => {
            match get_hostname_from_ip(&ip) {
                None => print!("{}\n", ip),
                Some(hostname) => print!("{} [{}]\n", ip, hostname)
            }
            if ip == target_ip {
                return true
            }
        }
    }

    return false
}

fn space_format(len: usize, to_format: String) -> String {
    return format!("{}{}", " ".repeat(len - to_format.len()), to_format)
}