use std::net::IpAddr;
use dns_lookup::lookup_host;
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::transport::{TransportProtocol::Ipv4, transport_channel, TransportChannelType::Layer4, icmp_packet_iter};

use crate::error::Error;
use crate::ipresolver::get_ip_addr;
use crate::ping::Pinger;

pub fn trace(addr: String) -> Result<(), Error> {
    let ip = get_ip_addr(addr)?;

    let (tx, mut rx) = match transport_channel(1500, Layer4(Ipv4(Icmp))) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            let description = format!("Failed to establish transport stream!: {}", e);
            return Err(Error::new(description));
        }
    };
    let mut pinger = Pinger::new(tx, &mut rx)?;

    let _ = pinger.send(64, ip)?;
    println!("{:?}", pinger.receive());

    Ok(())
}