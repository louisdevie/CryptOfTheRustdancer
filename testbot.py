import socket

with socket.create_connection(("127.0.0.1", 54321)) as conn:
    conn.send(b"MAP\n")

    for i in range(37):
        print(conn.recv(37))
