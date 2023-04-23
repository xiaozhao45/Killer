from art import *
from colorama import init, Fore, Back, Style
import subprocess

print(Fore.WHITE + 'Done Start!')

tprint("K i l l e r")

print(Fore.RED + "[K] Attack A Computer On An Intranet Through Arp")
print(Fore.GREEN + """
[D] Propagate The File To The Target Computer
[S] Scan All IPs Of The Internal Network
[A] About The Program
[E] Exit The Program
""")

do = input(Fore.GREEN + "What You Want To Do? >>>")
if do == 'K':
    subprocess.run(['python', 'kill.py'])
elif do == 'S':
    import ip
    ip.Scan_for_IP
elif do == 'D':
    subprocess.run(['python', 'port.py'])
elif do == 'E':
    print("Bye!")
elif do == 'A':
    print("""
    GitHub : https://github.com/xiaozhao45/Killer""")
