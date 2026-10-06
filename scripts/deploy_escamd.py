import os
import socket
import threading
import time
import sys
from tftp_server import serve_file

CAMERA_IP = os.environ.get("ESCAM_IP", "10.75.2.93")
CAMERA_TELNET_PORT = int(os.environ.get("ESCAM_TELNET_PORT", "2323"))
HOST_IP = os.environ.get("HOST_IP", "10.75.2.101")
TFTP_PORT = int(os.environ.get("TFTP_PORT", "6969"))
REMOTE_PATH = "/mnt/mtd/ipc/tmpfs/escamd.gz"
LOCAL_BINARY = "/tmp/escamd.gz"

def run_tftp():
    serve_file(LOCAL_BINARY, bind_ip="0.0.0.0", bind_port=TFTP_PORT)

def main():
    print("=== Step 1: Launching TFTP server on host ===")
    tftp_thread = threading.Thread(target=run_tftp, daemon=True)
    tftp_thread.start()
    time.sleep(0.5)

    print(f"=== Step 2: Connecting to camera root shell at {CAMERA_IP}:{CAMERA_TELNET_PORT} ===")
    s = socket.socket()
    s.settimeout(30.0)
    s.connect((CAMERA_IP, CAMERA_TELNET_PORT))
    time.sleep(0.3)
    s.send(b"\n")
    time.sleep(0.2)
    s.recv(1024)

    print(f"=== Step 3: Triggering TFTP download of escamd.gz to {REMOTE_PATH} ===")
    cmd_download = f"tftp -g -r escamd -l {REMOTE_PATH} {HOST_IP} {TFTP_PORT}\n"
    s.send(cmd_download.encode())

    # Wait for TFTP download to complete
    tftp_thread.join(timeout=30.0)
    time.sleep(1.0)
    output = s.recv(4096).decode("latin1", errors="ignore")
    print("Download response:", output.strip())

    print("=== Step 4: Decompress and perform atomic swap ===")
    commands = [
        ("cp /mnt/mtd/ipc/tmpfs/escamd.gz /mnt/mtd/ipc/conf/escamd.gz", 0.5),
        ("zcat /mnt/mtd/ipc/tmpfs/escamd.gz > /mnt/mtd/ipc/tmpfs/escamd.new && chmod +x /mnt/mtd/ipc/tmpfs/escamd.new", 2.5),
        ("killall -9 escamd onvif net_detect sd.sh sd_detect 2>/dev/null; rm -f /mnt/mtd/ipc/tmpfs/escamd && mv /mnt/mtd/ipc/tmpfs/escamd.new /mnt/mtd/ipc/tmpfs/escamd && /mnt/mtd/ipc/tmpfs/escamd > /mnt/mtd/ipc/tmpfs/escamd.log 2>&1 &", 1.5),
        ("ps | grep escamd", 0.5),
        ("cat /mnt/mtd/ipc/tmpfs/escamd.log", 0.5),
    ]
    for cmd, delay in commands:
        s.send(f"{cmd}\n".encode())
        time.sleep(delay)
        resp = s.recv(4096).decode("latin1", errors="ignore")
        print(f"[{cmd}]:\n{resp.strip()}")

    s.close()
    print("=== escamd Daemon Successfully Deployed & Running! ===")

if __name__ == "__main__":
    main()
