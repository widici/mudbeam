use crate::error::Error;
use crate::util::get_ip_addr;
use crate::ping::{Pinger, PingResult};

pub fn trace(addr: String) -> Result<(), Error> {
    // Pinger setup
    let target_ip = get_ip_addr(addr)?;
    let mut pinger = Pinger::new(target_ip)?;

    // Traceroute command
    let mut ttl: u8 = 1;
    loop {
        let start = pinger.send(ttl)?;
        let response = pinger.receive(start).unwrap();

        println!(" {} {}", ttl, response);

        if let PingResult::Ok {ip, .. } = response {
            if ip == target_ip {
                break;
            }
        }

        ttl += 1;
    }

    Ok(())
}