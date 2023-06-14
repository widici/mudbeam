use crate::error::Error;
use crate::dns::get_ip_addr;
use crate::ping::{Pinger, PingResult};
use crate::util::summarize_responses;

pub fn trace(addr: String) -> Result<(), Error> {
    // Pinger setup
    let target_ip = get_ip_addr(addr)?;
    let mut pinger = Pinger::new(target_ip)?;

    // Traceroute command
    for ttl in 1..=64 {
        let mut responses: Vec<PingResult> = Vec::with_capacity(64*3);
        for _ in 0..3 {
            let start = pinger.send(ttl)?;
            let response = pinger.receive(start).unwrap();
            responses.push(response)
        }
        if summarize_responses(ttl, responses, target_ip) {
            println!("Trace complete!");
            break;
        }

        /*

        let formatted_ttl = format!("{}{}", " ".repeat(2 - ttl.to_string().len()), ttl);
        println!("{} {}", formatted_ttl, response);

        if let PingResult::Ok {ip, .. } = response {
            if ip == target_ip {
                break;
            }
        }
         */
    }

    Ok(())
}