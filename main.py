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
            messagebox.showinfo("致命错误！", "未安装WinPcap!")

# 关闭主窗口
root.destroy()

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
        Killer v2.0
        更新内容： 1.使用py2exe进行重打包
                   2.重写了kill.py和ip.py（也就是重写了攻击ip和ip扫描）
        GitHub : https://github.com/xiaozhao45/Killer""")
    else:
         print("错误")

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

               佛祖保佑         永无BUG

"""