use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes::EchoRequest};
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::transport::{TransportProtocol::Ipv4, transport_channel, TransportChannelType::Layer4};
use pnet::transport::{icmp_packet_iter, TransportReceiver, TransportSender};
use pnet::util::checksum;
use std::{net::IpAddr, time::{Duration, Instant}};
use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::util::get_url_from_ip;

pub struct Pinger {
    tx: TransportSender,
    rx: TransportReceiver,
    target_ip: IpAddr,
}

impl Pinger {
    pub fn new(target_ip: IpAddr) -> Result<Pinger, Error> {
        let (tx, rx) = match transport_channel(1500, Layer4(Ipv4(Icmp))) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => {
                let description = format!("Failed to establish transport stream!: {}", e);
                return Err(Error::new(description));
            }
        };

        Ok(Pinger { tx, rx, target_ip })
    }

    pub fn send(&mut self, ttl: u8) -> Result<Instant, Error> {
        let mut payload = [0u8; 64];

        let mut packet = MutableEchoRequestPacket::new(&mut payload).unwrap();
        packet.set_icmp_type(EchoRequest);
        packet.set_checksum(checksum(packet.packet(), 1));

        let tx = &mut self.tx;
        tx.set_ttl(ttl).unwrap();

        return match tx.send_to(packet, self.target_ip) {
            Ok(_) =>  Ok(Instant::now()),
            Err(e) => {
                let description = String::from(format!("Failed to send packet!: {}", e));
                Err(Error::new(description))
            }
        }
    }

    pub fn receive(&mut self, start: Instant) -> Result<PingResult, Error> {
        let timeout = Duration::from_millis(10);

        let mut rx_iter = icmp_packet_iter(&mut self.rx);
        while start.elapsed().as_secs() < 3 {
            match rx_iter.next_with_timeout(timeout) {
                Ok(Some((_, ip))) => {
                    let rtt = start.elapsed().as_millis();
                    let result = PingResult::Ok{ip, rtt};
                    return Ok(result);
                },
                Ok(None) => {
                    continue;
                },
                Err(e) => {
                    let description = format!("Something went wrong when receiving response!: {}", e);
                    return Err(Error::new(description));
                }
            }
        }

        return Ok(PingResult::Timeout)
    }
}

#[derive(Debug)]
pub enum PingResult {
    Ok {
        ip: IpAddr,
        rtt: u128,
    },
    Timeout,
}

impl Display for PingResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PingResult::Ok { ip, rtt } => {
                if let Some(hostname) = get_url_from_ip(ip) {
                    write!(f, "{} {}[{}]", rtt, hostname, ip)?
                } else {
                    write!(f, "{} {}", rtt, ip)?
                }
            },
            PingResult::Timeout => write!(f, "-")?,
        }

        Ok(())
    }
}