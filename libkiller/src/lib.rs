use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

const ARP_ETHERTYPE: u16 = 0x0806;
const ARP_REQUEST: u16 = 0x0001;
const ARP_REPLY: u16 = 0x0002;

// arp header
#[repr(C, packed)]
struct ArpHdr {
    hwtype: u16,
    proto: u16,
    hwlen: u8,
    protolen: u8,
    opcode: u16,
    sender_hwaddr: [u8; 6],
    sender_protoaddr: [u8; 4],
    target_hwaddr: [u8; 6],
    target_protoaddr: [u8; 4],
}

impl ArpHdr {
    // Implement a method to write the bytes of `ArpHdr` to a buffer.
    fn write_to_buffer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.hwtype.to_be_bytes())?;
        writer.write_all(&self.proto.to_be_bytes())?;
        writer.write_all(&self.hwlen.to_be_bytes())?;
        writer.write_all(&self.protolen.to_be_bytes())?;
        writer.write_all(&self.opcode.to_be_bytes())?;
        writer.write_all(&self.sender_hwaddr)?;
        writer.write_all(&self.sender_protoaddr)?;
        writer.write_all(&self.target_hwaddr)?;
        writer.write_all(&self.target_protoaddr)?;
        Ok(())
    }

    // Implement a method to read the bytes of `ArpHdr` from a buffer.
    fn from_bytes(buf: &[u8]) -> Self {
        let mut offset = 0;
        let hwtype = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        offset += 2;
        let proto = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        offset += 2;
        let hwlen = buf[offset];
        offset += 1;
        let protolen = buf[offset];
        offset += 1;
        let opcode = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        offset += 2;
        let sender_hwaddr = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
        ];
        offset += 6;
        let sender_protoaddr = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        offset += 4;
        let target_hwaddr = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
        ];
        offset += 6;
        let target_protoaddr = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];

        ArpHdr {
            hwtype,
            proto,
            hwlen,
            protolen,
            opcode,
            sender_hwaddr,
            sender_protoaddr,
            target_hwaddr,
            target_protoaddr,
        }
    }
}

// Send an ARP request to discover the MAC address of a given IP address.
fn send_arp_request(iface: &str, target_ip: Ipv4Addr) -> Option<[u8; 6]> {
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))).ok()?;
    socket.set_read_timeout(Some(Duration::from_millis(1000))).ok()?;

    let mut buf = Vec::new(); // 使用 Vec 作为缓冲区
    let mut arphdr = ArpHdr {
        hwtype: 1,
        proto: 0x0800,
        hwlen: 6,
        protolen: 4,
        opcode: ARP_REQUEST,
        sender_hwaddr: [0; 6],
        sender_protoaddr: target_ip.octets(),
        target_hwaddr: [0; 6],
        target_protoaddr: [0; 4],
    };

    // 将 ArpHdr 写入缓冲区
    arphdr.write_to_buffer(&mut buf).unwrap_or_else(|_| panic!("Failed to write ArpHdr to buffer"));

    // 发送数据
    socket.send_to(&buf, SocketAddr::from((target_ip, 68))).ok()?;

    let now = Instant::now();
    loop {
        let (len, src) = socket.recv_from(&mut buf).ok()?;
        let elapsed = now.elapsed().as_millis();

        if elapsed >= 1000 {
            break;
        }

        if len < 28 {
            continue;
        }

        let h = ArpHdr::from_bytes(&buf[..28]); // 只解析前 28 字节
        if h.hwtype == 1 && h.proto == 0x0800 && h.hwlen == 6 && h.protolen == 4 && h.opcode == ARP_REPLY {
            return Some(h.sender_hwaddr);
        }
    }

    None
}

// Send an ARP reply to poison the ARP cache of a given target IP address.
pub fn send_arp_reply(iface: &str, target_ip: Ipv4Addr, target_mac: [u8; 6], sender_ip: Ipv4Addr, sender_mac: [u8; 6]) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;

    let mut buf = Vec::new(); // 使用 Vec 作为缓冲区
    let mut arphdr = ArpHdr {
        hwtype: 1,
        proto: 0x0800,
        hwlen: 6,
        protolen: 4,
        opcode: ARP_REPLY,
        sender_hwaddr: sender_mac,
        sender_protoaddr: sender_ip.octets(),
        target_hwaddr: target_mac,
        target_protoaddr: target_ip.octets(),
    };

    // 将 ArpHdr 写入缓冲区
    arphdr.write_to_buffer(&mut buf)?;

    // 发送数据
    socket.send_to(&buf, SocketAddr::from((target_ip, 68)))?;

    Ok(())
}
