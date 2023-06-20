use crate::error::Error;
use crate::dns::get_ip_addr;
use crate::parsing::Args;
use crate::ping::{Pinger, PingResult};
use crate::util::summarize_responses;

pub fn trace(args: Args) -> Result<(), Error> {
    // Pinger setup
    let target_ip = get_ip_addr(args.addr)?;
    let mut pinger = Pinger::new(target_ip, args.timeout)?;

    // Traceroute command
    for ttl in args.start_ttl..=args.max_ttl {
        let mut responses: Vec<PingResult> = Vec::with_capacity(64*3);
        for _ in 0..args.n_attempts {
            let start = pinger.send(ttl)?;
            let response = pinger.receive(start).unwrap();
            responses.push(response)
        }
        if summarize_responses(ttl, responses, target_ip) {
            println!("Trace complete!");
            break;
        }
    }

    Ok(())
}