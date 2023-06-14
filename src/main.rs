mod traceroute;
mod ping;
mod error;
mod dns;
mod parsing;
mod util;

use traceroute::trace;

fn main() {
    let args = parsing::parse();
    let ip = args.get_one::<String>("IP").unwrap();
    match trace(ip.to_owned()) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }
}
