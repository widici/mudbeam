use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes::EchoRequest, echo_reply::EchoReplyPacket};
use pnet::packet::{ip::IpNextHeaderProtocols::Icmp, Packet};
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer4, TransportProtocol::Ipv4};
use pnet::util::checksum;
use std::{net::IpAddr, time::Duration};
use tokio::runtime::Runtime;

pub fn ping(ip: IpAddr) {
    let (mut tx, mut rx) = match transport_channel(1500, Layer4(Ipv4(Icmp))) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            panic!("Failed to establish transport stream!: {:?}", e)
        }
    };

    let mut buffer = [0u8; 64];
    let mut receiver = icmp_packet_iter(&mut rx);

    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_icmp_type(EchoRequest);
    packet.set_checksum(checksum(packet.packet(), 1));

    tx.set_ttl(64).unwrap();
    match tx.send_to(packet, ip) {
        Ok(bytes) => println!("Success! sent {:?} bytes", bytes),
        Err(_) => panic!("Failed to send packet!")
    }

    let runtime = Runtime::new().unwrap();
    let timeout = Duration::from_secs(1);

    let future_receiver = async {
        loop {
            match receiver.next_with_timeout(timeout) {
                Ok(Some((pkt, ip))) => {
                    println!("Pong! {:?} {}", EchoReplyPacket::new(pkt.packet()), ip);
                    break;
                },
                Ok(None) => {
                    println!("1");
                    continue;
                },
                Err(e) => {
                    panic!("Error: {:?}", e)
                },
            }
        }
    };

    runtime.block_on(future_receiver);
}