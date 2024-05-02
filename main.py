from art import * # type: ignore
from colorama import Fore # type: ignore
from PyQt5.QtWidgets import QApplication, QMessageBox,QWidget
import os
import sys
import tiptext
import random




##Initiation phase  
##############################################################
#Clear the screen and prompt to complete startup
os.system('cls' if os.name == 'nt' else 'clear')
print(Fore.WHITE + 'Done Start!')



# Variable names in the TipText module
variable_names = ['tiptext_01', 'tiptext_02', 'tiptext_03', 'tiptext_04', 'tiptext_05',
                  'tiptext_06', 'tiptext_07', 'tiptext_08', 'tiptext_09', 'tiptext_10']

# Randomly select a variable name from a TipText module  
selected_variable_name = random.choice(variable_names)

# Use getattr to obtain variable values in module tiptext  
selected_value = getattr(tiptext, selected_variable_name)


#Create PyQt window for QMessageBox  
from PyQt5.QtWidgets import QMessageBox
app = QApplication(sys.argv)
app.setQuitOnLastWindowClosed(False)  
window = QWidget()
window.setVisible(False)  # Ensure that the window is not visible
##############################################################







##Environmental testing  
##############################################################
if not os.path.exists("C:/Program Files (x86)/WinPcap"):
    msg = QMessageBox()
    msg.setIcon(QMessageBox.Critical)
    msg.setText("Error! WinPcap not installed! ")
    msg.setWindowTitle("Error")
    msg.exec_()
##############################################################







##Main program  
##############################################################

tprint("K i l l e r") # type: ignore

print(selected_value)
print("\n")

print(Fore.RED + "[K] Using Arp to attack a computer on an intranet  ")
print(Fore.GREEN + """
[S] Scan all active IPs in the internal network
[N] Obtain local IP, gateway, and MAC addresses 
[P] Scan open ports for specified LAN IPs 
[A] About this program  
[E] Exit program  
""")
while True:
    do = input(Fore.GREEN + "\nWhat are you going to do? Input letter >>>")
    do = do.upper()
    if do == 'K':
        import kill
    elif do == 'S':
        import ip
    elif do == 'D':
        print("Removed!")
    elif do == 'N':
        import net
    elif do == 'P':
        import port
    elif do == 'E':
        print('Done, exiting...')
        break
    elif do == 'A':
        print("""
        Killer v4.0 (English)
        Update content : 
                   1.Remove the function of sending files
                   2.Fix some errors in Kill functionality  
                   3.Update header text  
                   4.Add IP scanning function exit button  
                   5.Others

        GitHub : https://github.com/xiaozhao45/Killer""")
    else:
         print("Error!")

##############################################################
## Program End ###############################################
##############################################################