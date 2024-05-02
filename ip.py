import socket
from scapy.all import ARP, Ether, srp
import sys

######################
#IP Scan V4.0
######################
#################
#By xiaozhao45
#################



while True:
    target_ip = input("Type the gateway, for example: 192.168.1.0/24, enter A to automatically obtain it, and type E to exit   >>")
    target_ip = target_ip.upper()
    if target_ip == 'A':
        import net
    elif target_ip == '':
        print("Please enter the correct IP address")
    elif target_ip == 'E':
        print("Bye!")
        sys.exit()
    else:
        break

# 数据包
arp = ARP(pdst=target_ip)
ether = Ether(dst="ff:ff:ff:ff:ff:ff")
packet = ether/arp

# 发数据包
result = srp(packet, timeout=3, verbose=0)[0]

# 处理
devices = []
for sent, received in result:
    devices.append({'ip': received.psrc, 'mac': received.hwsrc})

# 输出                                                                                                                                                                                          
for device in devices:
    if device['ip'] == '192.168.1.1':
        hostname = 'Gateway / Router'
    else:
        try:
            # 设备名
            hostname = socket.gethostbyaddr(device['ip'])[0]
        except:
            hostname = 'Unknown'
    print(f"IP: {device['ip']}, MAC: {device['mac']}, DeviceName: {hostname}")

print('Done, exiting...')