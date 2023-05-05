# from scapy.all import ARP, Ether, srp
# def Scan_for_IP():
    
#     while True:
#         target_ip = input("输入网段 (比如 192.168.0.0/24): ")
#         if "/" in target_ip:
#             break
#         else:
#             print("IP 地址格式无效。请包括网络掩码。")

    
#     arp = ARP(pdst=target_ip)
#     ether = Ether(dst="ff:ff:ff:ff:ff:ff")
#     packet = ether/arp

    
#     result = srp(packet, timeout=3, verbose=0)[0]

#     # 提取MAC
#     devices = []
#     for sent, received in result:
#         devices.append({'ip': received.psrc, 'mac': received.hwsrc})

#     # 输出
#     for device in devices:
#         print(f"IP: {device['ip']}, MAC: {device['mac']}")

# Scan_for_IP()



from scapy.all import ARP, Ether, srp

# 创建ARP请求包
arp_request = ARP(pdst=input("Type gateway and append /24 >>>"))

# 将ARP请求包封装在以太网帧中
ether = Ether(dst="ff:ff:ff:ff:ff:ff")

# 将ARP请求包发送到网络并等待响应
packet = ether / arp_request
result = srp(packet, timeout=3, verbose=0)[0]

# 遍历响应列表并输出设备名、IP地址和MAC地址
devices = []
for sent, received in result:
    devices.append({'ip': received.psrc, 'mac': received.hwsrc})

for device in devices:
    try:
        hostname = str(socket.gethostbyaddr(device['ip'])[0])
    except socket.herror:
        hostname = "Unknown"
    print(f"name: {hostname} | IP: {device['ip']} | MAC: {device['mac']}")
