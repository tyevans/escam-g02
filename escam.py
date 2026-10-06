#!/usr/bin/env python3
"""
ESCAM G02 IP Camera CLI & Reverse Engineering Utility.

Provides direct access to the camera's HTTP CGI API for PTZ motor control,
snapshots, RTSP stream launching, and Wi-Fi / system querying without any
proprietary apps or cloud services.
"""

import argparse
import os
import subprocess
import sys
import urllib.parse
import urllib.request
import base64

DEFAULT_IP = os.environ.get("ESCAM_IP", "10.75.2.138")
DEFAULT_USER = os.environ.get("ESCAM_USER", "admin")
DEFAULT_PASS = os.environ.get("ESCAM_PASS", "admin")


def make_request(ip: str, user: str, password: str, path: str) -> str:
    url = f"http://{ip}{path}"
    req = urllib.request.Request(url)
    auth_header = base64.b64encode(f"{user}:{password}".encode()).decode("ascii")
    req.add_header("Authorization", f"Basic {auth_header}")
    try:
        with urllib.request.urlopen(req, timeout=5) as resp:
            return resp.read().decode("utf-8", errors="replace")
    except Exception as e:
        print(f"[-] Request failed to {url}: {e}", file=sys.stderr)
        return ""


def download_file(ip: str, user: str, password: str, path: str, dest: str) -> bool:
    url = f"http://{ip}{path}"
    req = urllib.request.Request(url)
    auth_header = base64.b64encode(f"{user}:{password}".encode()).decode("ascii")
    req.add_header("Authorization", f"Basic {auth_header}")
    try:
        with urllib.request.urlopen(req, timeout=5) as resp, open(dest, "wb") as f:
            f.write(resp.read())
        print(f"[+] Snapshot saved to: {dest}")
        return True
    except Exception as e:
        print(f"[-] Download failed: {e}", file=sys.stderr)
        return False


def ptz_control(ip: str, user: str, password: str, action: str):
    path = f"/cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act={urllib.parse.quote(action)}"
    res = make_request(ip, user, password, path)
    print(f"[PTZ {action}]: {res.strip()}")


def main():
    parser = argparse.ArgumentParser(description="ESCAM G02 Camera Utility")
    parser.add_argument("--ip", default=DEFAULT_IP, help=f"Camera IP (default: {DEFAULT_IP})")
    parser.add_argument("--user", default=DEFAULT_USER, help="Username (default: admin)")
    parser.add_argument("--password", default=DEFAULT_PASS, help="Password (default: admin)")

    subparsers = parser.add_subparsers(dest="command", required=True)

    # PTZ command
    ptz_parser = subparsers.add_parser("ptz", help="Control Pan/Tilt motors")
    ptz_parser.add_argument("action", choices=["up", "down", "left", "right", "home", "stop", "hscan", "vscan"])

    # Snapshot command
    snap_parser = subparsers.add_parser("snap", help="Capture a 720p JPEG snapshot")
    snap_parser.add_argument("--output", "-o", default="snapshot.jpg", help="Output file path")

    # Status command
    subparsers.add_parser("status", help="Query network and system status")

    # Scan wifi command
    subparsers.add_parser("scan-wifi", help="Trigger camera Wi-Fi scan")

    # Stream command
    stream_parser = subparsers.add_parser("stream", help="Display RTSP stream with ffplay/vlc")
    stream_parser.add_argument("--player", choices=["ffplay", "vlc"], default="ffplay")

    args = parser.parse_args()

    if args.command == "ptz":
        ptz_control(args.ip, args.user, args.password, args.action)

    elif args.command == "snap":
        download_file(args.ip, args.user, args.password, "/tmpfs/auto.jpg", args.output)

    elif args.command == "status":
        raw = make_request(args.ip, args.user, args.password, "/cgi-bin/hi3510/param.cgi?cmd=getnetattr&cmd=gethip2pattr")
        print(raw)

    elif args.command == "scan-wifi":
        raw = make_request(args.ip, args.user, args.password, "/cgi-bin/hi3510/param.cgi?cmd=searchwireless")
        print(raw)

    elif args.command == "stream":
        rtsp_url = f"rtsp://{args.user}:{args.password}@{args.ip}:554/11"
        print(f"[*] Launching stream: {rtsp_url}")
        if args.player == "ffplay":
            subprocess.run(["ffplay", "-rtsp_transport", "tcp", rtsp_url])
        else:
            subprocess.run(["vlc", rtsp_url])


if __name__ == "__main__":
    main()
