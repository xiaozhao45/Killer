import socket
import uuid
import sys

######################
#Network Panel V4.0
######################
#################
#By xiaozhao45
#################


def get_ip():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect(("8.8.8.8", 80))
    ip = s.getsockname()[0]
    s.close()
    return ip

s = get_ip()
parts = s.split(".")
parts[3] = "0/24"
result = ".".join(parts)


def get_mac_address():
    mac = uuid.UUID(int=uuid.getnode()).hex[-12:]
    return ":".join([mac[e:e+2] for e in range(0, 11, 2)])



MAC = get_mac_address()
IP = get_ip()
G_IP = result




print ("╔════════ Network Panel ═════════╗")
print ("║ 当前设备IP :" + IP + "     ║")
print ("║ 网关IP     :" + result + "    ║")
print ("║ MAC地址    :" + MAC + "  ║")
print ("╚════════════════════════════════╝")

print ("\n")
print('完毕，正在退出...')
sys.exit