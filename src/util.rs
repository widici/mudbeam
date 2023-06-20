use std::net::IpAddr;
use crate::dns::get_hostname_from_ip;
use crate::ping::PingResult;

pub fn summarize_responses(ttl: u8, max_ttl: u8, responses: Vec<PingResult>, target_ip: IpAddr, timeout: u64) -> bool {
    let mut ip_addr: Option<IpAddr> = None;
    let len = (timeout * 1000).to_string().len();
    print!("{}  ", space_format(max_ttl.to_string().len(), ttl.to_string()));
    for response in responses {
        match response {
            PingResult::Ok { ip, rtt } => {
                print!("{} ms  ", space_format(len, rtt.to_string()));
                if ip_addr.is_none() {
                    ip_addr = Some(ip)
                }
            }
            PingResult::Timeout => print!("{}", space_center(len + 4, "*"))
        }
    }

    match ip_addr {
        None => print!("Timed out.\n"),
        Some(ip) => {
            match get_hostname_from_ip(&ip) {
                None => print!("{}\n", ip),
                Some(hostname) => print!("{} [{}]\n", ip, hostname)
            }
            return ip == target_ip
        }
    }

    return false
}

fn space_format(len: usize, to_format: String) -> String {
    return format!("{}{}", " ".repeat(len - to_format.len()), to_format)
}

fn space_center(len: usize, to_center: &str) -> String {
    let half = len / 2;
    let sides: (String, String) = if len % 2 == 0 {
            (" ".repeat(half), " ".repeat(half))
        } else {
            (" ".repeat(half), " ".repeat(half + 1))
        };

    return format!("{}{}{}", sides.0, to_center, sides.1)
}