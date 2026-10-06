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
REMOTE_PATH = "/mnt/mtd/ipc/tmpfs/escamd"
LOCAL_BINARY = "target/arm-unknown-linux-musleabi/release/escamd"

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
    s.send(b"killall -9 escamd 2>/dev/null\nrm -f /mnt/mtd/ipc/tmpfs/escamd.log\n")
    time.sleep(0.3)
    s.recv(1024)

    print(f"=== Step 3: Triggering TFTP download to {REMOTE_PATH} ===")
    cmd_download = f"tftp -g -r escamd -l {REMOTE_PATH} {HOST_IP} {TFTP_PORT}\n"
    s.send(cmd_download.encode())

    # Wait for TFTP download to complete
    tftp_thread.join(timeout=45.0)
    time.sleep(1.0)
    output = s.recv(4096).decode("latin1", errors="ignore")
    print("Download response:", output.strip())

    print("=== Step 4: Chmod and launch escamd in background on port 8080 ===")
    s.send(f"chmod +x {REMOTE_PATH}\n".encode())
    time.sleep(0.2)
    s.send(f"ESCAM_BIND=0.0.0.0:8080 {REMOTE_PATH} > /mnt/mtd/ipc/tmpfs/escamd.log 2>&1 &\n".encode())
    time.sleep(1.0)

    s.send(b"ps | grep escamd\n")
    time.sleep(0.5)
    ps_output = s.recv(2048).decode("latin1", errors="ignore")
    print("Running process:", ps_output.strip())

    s.send(b"cat /mnt/mtd/ipc/tmpfs/escamd.log\n")
    time.sleep(0.5)
    log_output = s.recv(2048).decode("latin1", errors="ignore")
    print("Daemon Log:\n", log_output.strip())

    s.close()
    print("=== escamd Daemon Successfully Deployed & Running! ===")

if __name__ == "__main__":
    main()
