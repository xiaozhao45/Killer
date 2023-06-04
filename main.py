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

from PyQt5.QtWidgets import QMessageBox

if not os.path.exists("C:/Program Files (x86)/WinPcap"):
    msg = QMessageBox()
    msg.setIcon(QMessageBox.Critical)
    msg.setText("错误！未安装WinPcap！")
    msg.setWindowTitle("错误")
    msg.exec_()


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
    elif do == 'D':
        import send
    elif do == 'E':
        print("Bye!")
        break
    elif do == 'A':
        print("""
        Killer v3.0
        更新内容： 1.优化了使用体验和逻辑
                   2.更人性化的注释（
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