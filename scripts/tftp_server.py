#!/usr/bin/env python3
import socket
import struct
import sys
import time

def serve_file(filepath, bind_ip="0.0.0.0", bind_port=6969):
    with open(filepath, "rb") as f:
        file_data = f.read()

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((bind_ip, bind_port))
    sock.settimeout(15.0)

    print(f"TFTP Server listening on {bind_ip}:{bind_port}, serving {filepath} ({len(file_data)} bytes)...")

    # Wait for RRQ (Read Request)
    data, client_addr = sock.recvfrom(512)
    opcode = struct.unpack("!H", data[:2])[0]
    if opcode != 1: # 1 = RRQ
        print(f"Unexpected opcode: {opcode}")
        return

    # Extract filename
    filename = data[2:].split(b"\0")[0].decode(errors="ignore")
    print(f"Received RRQ from {client_addr} for '{filename}'")

    block_size = 512
    block_num = 1
    offset = 0
    sock.settimeout(2.0)

    while True:
        chunk = file_data[offset:offset + block_size]
        packet = struct.pack("!HH", 3, block_num) + chunk

        # Send with retries
        success = False
        for attempt in range(10):
            sock.sendto(packet, client_addr)
            try:
                ack_data, sender = sock.recvfrom(512)
                if sender != client_addr:
                    continue
                ack_opcode, ack_block = struct.unpack("!HH", ack_data[:4])
                if ack_opcode == 4 and ack_block == block_num:
                    success = True
                    break
                elif ack_opcode == 4 and ack_block == (block_num - 1) % 65536:
                    # Duplicate ACK for previous block, loop to retransmit
                    time.sleep(0.01)
                    continue
                elif ack_opcode == 5:
                    print(f"TFTP Error from client: {ack_data[4:].decode(errors='ignore')}")
                    break
            except socket.timeout:
                continue

        if not success:
            print(f"Failed to transfer block {block_num} after retries")
            break

        offset += len(chunk)
        if len(chunk) < block_size:
            # Transfer complete!
            print(f"Transfer of {len(file_data)} bytes complete!")
            break

        block_num = (block_num + 1) % 65536

    sock.close()

if __name__ == "__main__":
    filepath = sys.argv[1] if len(sys.argv) > 1 else "target/arm-unknown-linux-musleabi/release/motor-test"
    serve_file(filepath)
