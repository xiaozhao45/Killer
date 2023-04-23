from art import *
from colorama import init, Fore, Back, Style
import subprocess

print(Fore.WHITE + 'Done Start!')

tprint("K i l l e r")

print(Fore.RED + "[K] 使用Arp攻击一个内网的计算机")
print(Fore.GREEN + """
[D] 发送文件到内网的计算机
[S] 扫描所有内网的活跃IP
[A] 关于这个程序
[E] 退出程序
""")
while True:
    do = input(Fore.GREEN + "\n你要做什么? >>>")
    if do == 'K':
        import kill
    elif do == 'S':
        import ip
        ip.Scan_for_IP
    elif do == 'D':
        import send
    elif do == 'E':
        print("Bye!")
        break
    elif do == 'A':
        print("""
        GitHub : https://github.com/xiaozhao45/Killer""")
