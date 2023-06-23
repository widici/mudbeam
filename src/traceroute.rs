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
        if summarize_responses(ttl, args.max_ttl, responses, target_ip, args.timeout) {
            println!("Trace complete!");
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::Args;

    #[test]
    fn trace_test() {
        for addr in ["localhost", "google.com", "yahoo.co.jp", "github.com", "1.1.1.1"] {
            let args = Args {
                addr: addr.to_string(),
                max_ttl: 64,
                start_ttl: 1,
                timeout: 1,
                n_attempts: 1,
            };

            assert!(trace(args).is_ok())
        }
    }
}