# How to Cross-Compile and Deploy to ESCAM G02

This guide walks through cross-compiling static Rust binaries for the camera's ARMv6 core and deploying them over Wi-Fi.

---

## 1. Prerequisites

Add the static ARMv6 musl target to your Rust toolchain:

```bash
rustup target add arm-unknown-linux-musleabi
```

---

## 2. Compile `motor-test` Proof of Concept

```bash
cargo build --release --target arm-unknown-linux-musleabi --bin motor-test
```

The output binary is located at:
`target/arm-unknown-linux-musleabi/release/motor-test`

---

## 3. Upload to Camera RAM

Upload the binary into `/tmpfs` via HTTP PUT using the camera's default credentials:

```bash
curl -u admin:admin -T target/arm-unknown-linux-musleabi/release/motor-test http://10.75.2.93/tmpfs/motor-test
```

Alternatively, use the helper script:

```bash
./scripts/deploy-motor-test.sh 10.75.2.93 2323
```

---

## 4. Execute via Root Telnet Shell

Connect to the unauthenticated root debug shell:

```bash
telnet 10.75.2.93 2323
```

Make the binary executable and run:

```bash
chmod +x /tmpfs/motor-test
/tmpfs/motor-test
```

Observe the pan and tilt motors sweeping smoothly with S-curve acceleration!
