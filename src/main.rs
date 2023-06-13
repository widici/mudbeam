mod traceroute;
mod ping;
mod error;
mod util;
mod parsing;

use traceroute::trace;

fn main() {
    let args = parsing::parse();
    let ip = args.get_one::<String>("IP").unwrap();
    trace(ip.to_owned()).unwrap();
}
