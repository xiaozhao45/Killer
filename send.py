import socket

# 设定接收数据的地址和端口
ip = input('IP 是? >>>')
port = input('端口是? >>>')

# 指定文件路径和名称
filename = input('文件是? >>>')

# 创建socket对象
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# 建立连接
s.connect((ip, port))

# 发送文件内容
with open(filename, 'rb') as f:
    data = f.read()
    s.sendall(data)

# 关闭连接
s.close()
