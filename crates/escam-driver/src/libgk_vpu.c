#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

// Direct ARM EABI System Calls (Zero libc dependencies)
static inline long sys_call1(long nr, long a) {
    register long r7 __asm__("r7") = nr;
    register long r0 __asm__("r0") = a;
    __asm__ __volatile__("swi 0" : "=r"(r0) : "r"(r7), "r"(r0) : "memory");
    return r0;
}

static inline long sys_call2(long nr, long a, long b) {
    register long r7 __asm__("r7") = nr;
    register long r0 __asm__("r0") = a;
    register long r1 __asm__("r1") = b;
    __asm__ __volatile__("swi 0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1) : "memory");
    return r0;
}

static inline long sys_call3(long nr, long a, long b, long c) {
    register long r7 __asm__("r7") = nr;
    register long r0 __asm__("r0") = a;
    register long r1 __asm__("r1") = b;
    register long r2 __asm__("r2") = c;
    __asm__ __volatile__("swi 0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2) : "memory");
    return r0;
}

static void safe_log(const char *msg) {
    int len = 0;
    while (msg[len]) len++;
    sys_call3(4, 2, (long)msg, len); // __NR_write to stderr
}

// Intercept bind: confine ALL sockets strictly to 127.0.0.1 (Localhost Loopback Only)
int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    if (addr && addr->sa_family == AF_INET) {
        struct sockaddr_in in;
        const struct sockaddr_in *orig = (const struct sockaddr_in *)addr;
        in = *orig;
        in.sin_addr.s_addr = 0x0100007f; // 127.0.0.1 loopback
        safe_log("[libgk_vpu] Confined bind() strictly to 127.0.0.1 loopback\n");
        return sys_call3(282, sockfd, (long)&in, sizeof(in)); // __NR_bind = 282
    }
    return sys_call3(282, sockfd, (long)addr, addrlen);
}

// Intercept connect: block all outbound WAN / cloud / P2P call-homes
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    if (addr && addr->sa_family == AF_INET) {
        const struct sockaddr_in *orig = (const struct sockaddr_in *)addr;
        if (orig->sin_addr.s_addr != 0x0100007f) {
            safe_log("[libgk_vpu] Blocked outbound cloud/P2P connect()\n");
            return -1; // Silently drop all cloud calls
        }
    }
    return sys_call3(283, sockfd, (long)addr, addrlen); // __NR_connect = 283
}

int _init(void) {
    safe_log("\n=======================================================\n");
    safe_log("🚀 libgk_vpu (Zero-Vendor Isolation & Loopback Jailing)\n");
    safe_log("   - Outbound Cloud/P2P: 100% BLOCKED\n");
    safe_log("   - All Inbound Services: JAILED TO 127.0.0.1 LOOPBACK\n");
    safe_log("   - External Network: ZERO VENDOR PORTS EXPOSED\n");
    safe_log("=======================================================\n\n");
    return 0;
}
