from art import * # type: ignore
from colorama import Fore # type: ignore
from PyQt5.QtWidgets import QApplication, QMessageBox,QWidget
import os
import sys
import tiptext
import random




##启动阶段
##############################################################
#清空屏幕并提示完成启动
os.system('cls' if os.name == 'nt' else 'clear')
print(Fore.WHITE + 'Done Start!')



# TipText 模块中的变量名
variable_names = ['tiptext_01', 'tiptext_02', 'tiptext_03', 'tiptext_04', 'tiptext_05',
                  'tiptext_06', 'tiptext_07', 'tiptext_08', 'tiptext_09', 'tiptext_10']

# 随机选择一个TipText 模块中的变量名
selected_variable_name = random.choice(variable_names)

# 使用 getattr 来获取模块 tiptext 中的变量值
selected_value = getattr(tiptext, selected_variable_name)


#PyQt窗口的创建，用于QMessageBox
from PyQt5.QtWidgets import QMessageBox
app = QApplication(sys.argv)
app.setQuitOnLastWindowClosed(False)  
window = QWidget()
window.setVisible(False)  # 确保窗口不可见
##############################################################







##环境检测
##############################################################
if not os.path.exists("C:/Program Files (x86)/WinPcap"):
    msg = QMessageBox()
    msg.setIcon(QMessageBox.Critical)
    msg.setText("错误！未安装WinPcap！")
    msg.setWindowTitle("错误")
    msg.exec_()
##############################################################







##主程序
##############################################################

tprint("K i l l e r") # type: ignore

print(selected_value)
print("\n")

print(Fore.RED + "[K] 使用Arp攻击一个内网的计算机")
print(Fore.GREEN + """
[S] 扫描所有内网的活跃IP
[N] 获取本机IP、网关和MAC地址
[P] 扫描指定局域网IP的开放端口
[A] 关于这个程序
[E] 退出程序
""")
while True:
    do = input(Fore.GREEN + "\n你要做什么? >>>")
    do = do.upper()
    if do == 'K':
        import kill
    elif do == 'S':
        import ip
    elif do == 'D':
        print("已被移除！")
    elif do == 'N':
        import net
    elif do == 'P':
        import port
    elif do == 'E':
        print('完毕，正在退出...')
        break
    elif do == 'A':
        print("""
        Killer v4.0 (Chinese)
        更新内容： 1.移除发送文件功能
                   2.修复Kill功能的部分错误
                   3.更新表头文本
                   4.增加IP扫描功能退出键
                   5.其他

        GitHub : https://github.com/xiaozhao45/Killer""")
    else:
         print("错误")

##############################################################
##程序结束####################################################
##############################################################