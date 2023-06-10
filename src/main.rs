mod traceroute;
mod ping;
mod error;
mod ipresolver;

use traceroute::trace;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(ip) = args.get(1).cloned() {
        trace(ip);
    }

}
