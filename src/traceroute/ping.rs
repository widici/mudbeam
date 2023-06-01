use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes::EchoRequest, echo_reply::EchoReplyPacket};
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer4, TransportProtocol::Ipv4};
use std::net::{IpAddr, Ipv4Addr};
use pnet::packet::Packet;

pub fn ping(ip: String) {
    let (mut tx, mut rx) = match transport_channel(1096, Layer4(Ipv4(Icmp))) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            panic!("Failed to establish transport stream!: {:?}", e)
        }
    };

    let mut buffer = [0u8; 64];
    let destination: IpAddr = IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap());
    let mut receiver = icmp_packet_iter(&mut rx);

    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_icmp_type(EchoRequest);

    match tx.send_to(packet, destination) {
        Ok(bytes) => println!("Success! sent {:?} bytes", bytes),
        Err(_) => eprintln!("Failed to send packet!")
    }

    loop {
        match receiver.next() {
            Ok((pkt, ip)) => {
                println!("Pong! {:?} {}", EchoReplyPacket::new(pkt.packet()).unwrap(), ip);
                break;
            }
            Err(e) => eprintln!("Error: {:?}", e)
        }
    }
}