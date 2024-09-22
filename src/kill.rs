

//狗屎代码别看啊TAT

use pcap::{Device, Packet};
use std::net::Ipv4Addr;

fn main() {

    let interface = Device::lookup().unwrap();
    let mut cap = interface.open().unwrap();


    let sender_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]; // 我方 MAC 
    let sender_ip = Ipv4Addr::new(192, 168, 1, 1); // 我方 IP 
    let target_mac = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff]; // 目标 MAC 
    let target_ip = Ipv4Addr::new(192, 168, 1, 2); // 目标 IP 

    let mut arp_packet = vec![];
    arp_packet.extend_from_slice(&target_mac);
    arp_packet.extend_from_slice(&sender_mac);
    arp_packet.extend_from_slice(&[0x08, 0x06]); 
    arp_packet.extend_from_slice(&[0x00, 0x01]); 
    arp_packet.extend_from_slice(&[0x08, 0x00]); 
    arp_packet.push(6);
    arp_packet.push(4); 
    arp_packet.extend_from_slice(&[0x00, 0x01]); 
    arp_packet.extend_from_slice(&sender_mac);
    arp_packet.extend_from_slice(&sender_ip.octets());
    arp_packet.extend_from_slice(&target_mac);
    arp_packet.extend_from_slice(&target_ip.octets());


    cap.send_packet(&arp_packet).unwrap();

    println!("ARP 请求包已发送！");
}
