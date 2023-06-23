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