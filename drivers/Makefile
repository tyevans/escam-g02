# ==============================================================================
# ESCAM G02 - Open-Source Motor & GKIO Driver Makefile
# Governed by ADR-0026 and TASK-0040
# ==============================================================================

obj-m += escam_motor.o

KDIR ?= /lib/modules/$(shell uname -r)/build
CROSS_COMPILE ?= arm-linux-musleabi-
ARCH ?= arm

all:
	$(MAKE) ARCH=$(ARCH) CROSS_COMPILE=$(CROSS_COMPILE) -C $(KDIR) M=$(PWD) modules

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
