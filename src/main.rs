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
    use crate::dns::{get_ip_addr, get_hostname_from_ip};
    use crate::parsing::Args;
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
            let ip: IpAddr = IpAddr::from_str(output).unwrap();
            assert_eq!(get_ip_addr(input.to_string()), Ok(ip));
        }
    }

    #[test]
    fn get_hostname_from_ip_test() {
        let test_cases: HashMap<&str, &Option<&str>> = [
            ("127.0.0.1", &Some("localhost")),
            ("1.1.1.1", &Some("one.one.one.one")),
            ("8.8.8.8", &Some("dns.google")),
            ("9.9.9.9", &Some("dns9.quad9.net")),
            ("0.0.0.0", &None)
        ]
            .into_iter()
            .collect();

        for (input, output) in test_cases {
            let ip: IpAddr = IpAddr::from_str(input).unwrap();
            assert_eq!(&get_hostname_from_ip(&ip).as_deref(), output)
        }
    }

    #[test]
    fn trace_test() {
        for addr in ["localhost", "yahoo.co.jp", "github.com", "gitlab.com", "1.1.1.1"] {
            let args = Args {
                addr: addr.to_string(),
                max_ttl: 64,
                start_ttl: 1,
                timeout: 1,
                n_attempts: 1,
            };

            let _ = trace(args);
        }
    }
}