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
        Ok(hostname) => Some(hostname),
        Err(_) => return None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::str::FromStr;

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
}