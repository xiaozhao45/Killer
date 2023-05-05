from art import *
from colorama import init, Fore, Back, Style
import subprocess
import os
import time
import os
from functools import partial


# 清空终端内容
os.system('cls' if os.name == 'nt' else 'clear')

print(Fore.WHITE + 'Done Start!')

import tkinter as tk
from tkinter import messagebox

# 创建主窗口
root = tk.Tk()
root.withdraw()

# 检查系统中是否安装了WinPcap
if not os.path.exists(r"C:\Program Files\WinPcap"):
            messagebox.showinfo("Fatal error!", "WinPcap is not installed!")

# 关闭主窗口
root.destroy()

tprint("K i l l e r")

print(Fore.RED + "[K] Use ARP to attack computers on an intranet")
print(Fore.GREEN + """
[D] Send files to computers on the intranet
[S] Scan all active IP addresses on the private network
[A] About this app
[E] Exit the program
""")
while True:
    do = input(Fore.GREEN + "\nWhat are you want to do? >>>")
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
        Killer v2.0
        What's new: 1. Repackage using py2exe
                   2. Rewrote kill.py and ip.py (i.e. rewrote attack IP and IP scanning)
        GitHub : https://github.com/xiaozhao45/Killer""")
    else:
         print("Error!")

"""

                       _oo0oo_
                      o8888888o
                      88" . "88
                      (| -_- |)
                      0\  =  /0
                    ___/`---'\___
                  .' \\|     |// '.
                 / \\|||  :  |||// \
                / _||||| -:- |||||- \
               |   | \\\  -  /// |   |
               | \_|  ''\---/''  |_/ |
               \  .-\__  '-'  ___/-. /
             ___'. .'  /--.--\  `. .'___
          ."" '<  `.___\_<|>_/___.' >' "".
         | | :  `- \`.;`\ _ /`;.`/ - ` : | |
         \  \ `_.   \_ __\ /__ _/   .-` /  /
     =====`-.____`.___ \_____/___.-`___.-'=====
                       `=---='


     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

               God forbid, there will never be loopholes
               Wait, is this God?

"""