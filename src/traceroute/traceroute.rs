use crate::traceroute::ping::ping;

pub fn trace(ip: String) {
    ping(ip)
}