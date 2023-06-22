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
    use crate::dns::get_ip_addr;

    #[test]
    fn get_ip_addr_test() {
        let test_cases: HashMap<&str, &str> = [
            ("localhost", "127.0.0.1"),
            ("127.0.0.1", "127.0.0.1"),
            ("1.1.1.1", "1.1.1.1"),
            ("example.com", "93.184.216.34")
        ]
            .into_iter()
            .collect();

        for (input, output) in test_cases {
            assert_eq!(get_ip_addr(input.to_string()), Ok(IpAddr::from_str(output).unwrap()));
        }
    }
}