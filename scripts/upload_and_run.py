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
REMOTE_PATH = "/mnt/mtd/ipc/tmpfs/motor-test"
LOCAL_BINARY = "target/arm-unknown-linux-musleabi/release/motor-test"

def run_tftp():
    serve_file(LOCAL_BINARY, bind_ip="0.0.0.0", bind_port=TFTP_PORT)

def main():
    print("=== Step 1: Launching TFTP server on host ===")
    tftp_thread = threading.Thread(target=run_tftp, daemon=True)
    tftp_thread.start()
    time.sleep(0.5)

    print(f"=== Step 2: Connecting to camera root shell at {CAMERA_IP}:{CAMERA_TELNET_PORT} ===")
    s = socket.socket()
    s.settimeout(20.0)
    s.connect((CAMERA_IP, CAMERA_TELNET_PORT))
    time.sleep(0.3)
    s.send(b"\n")
    time.sleep(0.2)
    s.recv(1024)

    print(f"=== Step 3: Triggering TFTP download to {REMOTE_PATH} ===")
    cmd_download = f"tftp -g -r motor-test -l {REMOTE_PATH} {HOST_IP} {TFTP_PORT}\n"
    s.send(cmd_download.encode())

    # Wait for TFTP download to complete
    tftp_thread.join(timeout=10.0)
    time.sleep(1.0)
    output = s.recv(4096).decode("latin1", errors="ignore")
    print("Download response:", output.strip())

    print("=== Step 4: Chmod and execute motor-test on camera hardware ===")
    s.send(f"chmod +x {REMOTE_PATH}\n".encode())
    time.sleep(0.2)
    s.send(f"{REMOTE_PATH}\n".encode())

    # Collect execution output
    start = time.time()
    while time.time() - start < 10.0:
        try:
            chunk = s.recv(4096)
            if chunk:
                sys.stdout.write(chunk.decode("latin1", errors="ignore"))
                sys.stdout.flush()
                if "complete!" in chunk.decode("latin1", errors="ignore"):
                    break
        except socket.timeout:
            break

    s.close()
    print("\n=== Hardware Execution Completed! ===")

if __name__ == "__main__":
    main()
