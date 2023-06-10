use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes::EchoRequest};
use pnet::packet::Packet;
use pnet::transport::{icmp_packet_iter, IcmpTransportChannelIterator, TransportReceiver, TransportSender};
use pnet::util::checksum;
use std::{net::IpAddr, time::{Duration, Instant}};
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
        let start = Instant::now();

        let rx_iter = &mut self.rx_iter;
        while start.elapsed().as_secs() < 5 {
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