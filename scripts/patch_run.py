import socket
import struct
import threading
import time
from pathlib import Path

def patch_run_file():
    content = Path('run.downloaded').read_text(encoding='latin1')
    target = '''loadmotor()
{
\tDEVTYPE=$CONF/config_devtype.ini
\tMOTORF=`grep motormode $DEVTYPE | awk -F "\\"" '{print $2}'`
\tPANS=`grep panrange $DEVTYPE | awk -F "\\"" '{print $2}'`
\tTILS=`grep tiltrange $DEVTYPE | awk -F "\\"" '{print $2}'`
\tinsmod $TARGET/modules/motor.ko pan_all=$PANS titl_all=$TILS spd_all=1100 spd_pan=1 spd_titl=1
}'''

    replacement = '''loadmotor()
{
\tDEVTYPE=$CONF/config_devtype.ini
\tMOTORF=`grep motormode $DEVTYPE | awk -F "\\"" '{print $2}'`
\tPANS=`grep panrange $DEVTYPE | awk -F "\\"" '{print $2}'`
\tTILS=`grep tiltrange $DEVTYPE | awk -F "\\"" '{print $2}'`
\tif [ -f /mnt/mtd/ipc/conf/escam_motor.ko ]; then
\t\techo "[init] Loading open-source escam_motor.ko..."
\t\trmmod gkio 2>/dev/null
\t\trmmod motor 2>/dev/null
\t\trm -f /dev/motor /dev/gkio
\t\tinsmod /mnt/mtd/ipc/conf/escam_motor.ko
\t\tmknod /dev/motor c 243 0 2>/dev/null
\t\tmknod /dev/gkio c 242 0 2>/dev/null
\telse
\t\tinsmod $TARGET/modules/motor.ko pan_all=$PANS titl_all=$TILS spd_all=1100 spd_pan=1 spd_titl=1
\tfi
}'''

    assert target in content, "Target block not found in run.downloaded!"
    patched = content.replace(target, replacement, 1)
    Path('run.patched').write_text(patched, encoding='latin1')
    print(f"run.patched written ({len(patched)} bytes)")

class SimpleTFTP:
    def __init__(self, host='0.0.0.0', port=6969):
        self.host = host
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind((self.host, self.port))
        self.running = True

    def serve(self):
        while self.running:
            try:
                data, addr = self.sock.recvfrom(516)
                if not data: continue
                opcode = struct.unpack('!H', data[:2])[0]
                if opcode == 1: # RRQ
                    threading.Thread(target=self.handle_rrq, args=(data, addr)).start()
            except:
                break

    def handle_rrq(self, data, addr):
        filename = data[2:].split(b'\x00')[0].decode('latin1')
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.settimeout(5)
        try:
            with open(filename, 'rb') as f:
                block = 1
                while True:
                    chunk = f.read(512)
                    pkt = struct.pack('!HH', 3, block) + chunk
                    s.sendto(pkt, addr)
                    ack, _ = s.recvfrom(516)
                    ack_op, ack_blk = struct.unpack('!HH', ack[:4])
                    if ack_op == 4 and ack_blk == block:
                        block = (block + 1) % 65536
                    if len(chunk) < 512:
                        break
            print(f"TFTP RRQ delivered {filename}")
        except Exception as e:
            print(f"TFTP RRQ error: {e}")
        finally:
            s.close()

def deploy_to_camera():
    srv = SimpleTFTP()
    t = threading.Thread(target=srv.serve, daemon=True)
    t.start()
    time.sleep(0.3)

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(10)
    s.connect(('10.75.2.93', 2323))
    time.sleep(0.3)
    # Download run.patched as /mnt/mtd/ipc/conf/run
    cmd = (
        b"tftp -g -l /mnt/mtd/ipc/conf/run -r run.patched 10.75.2.101 6969 && "
        b"chmod +x /mnt/mtd/ipc/conf/run && "
        b"sync && "
        b"ls -l /mnt/mtd/ipc/conf/run\n"
    )
    s.sendall(cmd)
    time.sleep(2)
    resp = s.recv(2048).decode('latin1', errors='ignore')
    print("Camera response:\n", resp)
    s.close()
    srv.running = False
    srv.sock.close()

if __name__ == '__main__':
    patch_run_file()
    deploy_to_camera()

