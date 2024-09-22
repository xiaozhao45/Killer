use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};

use pcap::Device;

pub fn pcap_open(
    device: Device,
    pcap_filter: &str,
) -> Result<pcap::Capture<pcap::Active>, pcap::Error> {
    let mut cap = device.open()?;
    cap.filter(pcap_filter, true)?;
    Ok(cap)
}


pub fn ip_forward(enable: bool) -> Result<(), Error> {
    let ipv4_fw_path = "/proc/sys/net/ipv4/ip_forward";
    let ipv4_fw_value = match enable {
        true => "1\n",
        false => "0\n",
    };

    let result = match OpenOptions::new().write(true).open(ipv4_fw_path) {
        Ok(mut f) => f.write_all(String::from(ipv4_fw_value).as_bytes()),
        Err(e) => panic!("无法打开 {}: {},请以root权限运行!", ipv4_fw_path, e),
    };
    println!("转发 IPv4 流量: {}", enable);
    result
}

pub fn get_interface_mac_addr(interface_name: &str) -> [u8; 6] {
    let path = format!("/sys/class/net/{}/address", interface_name);
    let mut mac_addr_buf = String::new();
    match File::open(&path) {
        Ok(mut f) => f.read_to_string(&mut mac_addr_buf).unwrap(),
        Err(e) => panic!(
            "无法读取Mac地址 {} : {}",
            path, e
        ),
    };
    string_to_mac(mac_addr_buf.trim())
}

pub fn mac_to_string(mac_addr: &[u8; 6]) -> String {
    mac_addr
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":")
}

pub fn string_to_mac(string: &str) -> [u8; 6] {
    let hx: Vec<u8> = string
        .split(':')
        .map(|b| u8::from_str_radix(b, 16).unwrap())
        .collect();
    if hx.len() != 6 {
        panic!(
            "长度无效: {}",
            string
        );
    }

    let mut mac_addr = [0u8; 6];
    for (&x, p) in hx.iter().zip(mac_addr.iter_mut()) {
        *p = x;
    }
    mac_addr
}
