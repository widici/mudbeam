use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes::EchoRequest, echo_reply::EchoReplyPacket};
use pnet::packet::{ip::IpNextHeaderProtocols::Icmp, Packet};
use pnet::transport::{icmp_packet_iter, IcmpTransportChannelIterator, transport_channel, TransportChannelType::Layer4, TransportProtocol::Ipv4, TransportReceiver, TransportSender};
use pnet::util::checksum;
use std::{net::IpAddr, time::Duration};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::runtime::Runtime;
use crate::error::Error;

pub struct Pinger<'a> {
    tx: TransportSender,
    rx_iter: IcmpTransportChannelIterator<'a>
}

impl<'a> Pinger<'a> {
    pub fn new(tx: TransportSender, rx: &'a mut TransportReceiver) -> Result<Pinger<'a>, Error> {
        let rx_iter = icmp_packet_iter(rx);

        Ok(Pinger { tx, rx_iter })
    }


    pub fn send(&mut self, ttl: u8, addr: IpAddr) -> Result<(), Error> {
        let mut payload = [0u8; 64];

        let mut packet = MutableEchoRequestPacket::new(&mut payload).unwrap();
        packet.set_icmp_type(EchoRequest);
        packet.set_checksum(checksum(packet.packet(), 1));

        let tx = &mut self.tx;
        tx.set_ttl(ttl).unwrap();

        return match tx.send_to(packet, addr) {
            Ok(_) =>  Ok(()),
            Err(e) => {
                let description = String::from(format!("Failed to send packet!: {}", e));
                Err(Error::new(description))
            }
        }
    }

    pub fn receive(&mut self) -> Result<PingResult, Error> {
        let timeout = Duration::from_millis(10);

        let mut rx_iter = &mut self.rx_iter;
        loop {
            match rx_iter.next_with_timeout(timeout) {
                Ok(Some((_, ip))) => {
                    let result = PingResult::new(ip);
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
    }

}

#[derive(Debug)]
pub struct PingResult {
    ip: IpAddr,
}

impl PingResult {
    pub fn new(ip: IpAddr) -> PingResult {
        return PingResult { ip }
    }
}