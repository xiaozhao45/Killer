import socket

######################
#Port Scan V4.0
######################
#################
#By xiaozhao45
#################


print("Port scan v4.0")

# Set the scanning port range and target IP address
ip = input("What IP address do you want to scan,input A to scan all IPs! >>>")
while True:
    if ip == "":
        ip = input("What IP address do you want to scan?The previous input is invalid! >>>")
    if ip == "A":
        import ip
    else:
        break
start_port = 1
end_port = 65535

# Loop scan all ports with specified IP addresses
for port in range(start_port, end_port+1):
    try:
        # Try to connect
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(1)  # timeout 1 second  
        s.connect((ip + str(port), port))
        s.close()

        # if open
        print('Port:', port, 'Is open!')
    except:
        pass

print('Done, exiting...')