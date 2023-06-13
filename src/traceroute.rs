use crate::error::Error;
use crate::util::get_ip_addr;
use crate::ping::{Pinger, PingResult};

pub fn trace(addr: String) -> Result<(), Error> {
    // Pinger setup
    let target_ip = get_ip_addr(addr)?;
    let mut pinger = Pinger::new(target_ip)?;

    // Traceroute command
    for ttl in 1..=64 {
        let start = pinger.send(ttl)?;
        let response = pinger.receive(start).unwrap();

        let formatted_ttl = format!("{}{}", " ".repeat(2 - ttl.to_string().len()), ttl);
        println!("{} {}", formatted_ttl, response);

        if let PingResult::Ok {ip, .. } = response {
            if ip == target_ip {
                break;
            }
        }
    }

    Ok(())
}