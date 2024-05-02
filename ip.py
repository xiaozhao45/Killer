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
    target_ip = input("键入网关，比如：192.168.1.0/24,输入A自动获取，退出请键入E  >>")
    target_ip = target_ip.upper()
    if target_ip == 'A':
        import net
    elif target_ip == '':
        print("请输入正确的IP！")
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
    print(f"IP: {device['ip']}, MAC: {device['mac']}, 设备名: {hostname}")

print('完毕，正在退出...')