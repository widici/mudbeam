mod traceroute;
mod ping;
mod error;
mod dns;
mod parsing;
mod util;

use traceroute::trace;

fn main() {
    let args = parsing::parse();

    match trace(args) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::str::FromStr;
    use std::collections::HashMap;
    use crate::dns::{get_ip_addr, get_ip_from_hostname};
    use crate::parsing::Args;
    use crate::ping::Pinger;
    use crate::traceroute::trace;

    #[test]
    fn get_ip_addr_test() {
        let test_cases: HashMap<&str, &str> = [
            ("localhost", "127.0.0.1"),
            ("127.0.0.1", "127.0.0.1"),
            ("1.1.1.1", "1.1.1.1"),
        ]
            .into_iter()
            .collect();

        for (input, output) in test_cases {
            assert_eq!(get_ip_addr(input.to_string()), Ok(IpAddr::from_str(output).unwrap()));
        }
    }

    #[test]
    fn trace_test() {
        for addr in ["localhost", "yahoo.co.jp", "github.com", "gitlab.com"] {
            let args = Args {
                addr: "localhost".to_string(),
                max_ttl: 64,
                start_ttl: 1,
                timeout: 1,
                n_attempts: 1,
            };

            assert!(trace(args).is_ok())
        }
    }
}