import socket

print("port [未完全开发，仅支持端口扫描]")

# 设定扫描端口范围和目标IP地址
ip = input("你想扫描的IP是? >>>")
start_port = 1
end_port = 65535

# 循环扫描指定IP地址的所有端口
for port in range(start_port, end_port+1):
    try:
        # 创建socket对象并尝试连接
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(1)  # 设置超时时间为1秒
        s.connect((ip + str(port), port))
        s.close()

        # 如果连接成功，说明该端口是开放的
        print('端口', port, '是开放的！')
    except:
        pass
