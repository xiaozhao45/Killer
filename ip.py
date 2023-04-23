from scapy.all import ARP, Ether, srp
def Scan_for_IP():
    
    while True:
        target_ip = input("Enter the IP address range to scan (e.g. 192.168.0.0/24): ")
        if "/" in target_ip:
            break
        else:
            print("Invalid IP address format. Please include the netmask.")

    
    arp = ARP(pdst=target_ip)
    ether = Ether(dst="ff:ff:ff:ff:ff:ff")
    packet = ether/arp

    
    result = srp(packet, timeout=3, verbose=0)[0]

    # 提取MAC
    devices = []
    for sent, received in result:
        devices.append({'ip': received.psrc, 'mac': received.hwsrc})

    # 输出
    for device in devices:
        print(f"IP: {device['ip']}, MAC: {device['mac']}")

Scan_for_IP()
