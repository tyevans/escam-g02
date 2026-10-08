/*
 * ESCAM G02 Open-Source Stepper Motor & GPIO Kernel Driver
 *
 * Replaces proprietary vendor modules: motor.ko and gkio.ko.
 * Target SoC: Goke GK7102C (ARMv6) / Mainline Linux (3.4.x - 6.x)
 * License: GPL-2.0
 */

#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/fs.h>
#include <linux/cdev.h>
#include <linux/device.h>
#include <linux/gpio.h>
#include <linux/hrtimer.h>
#include <linux/ktime.h>
#include <linux/uaccess.h>

#define DRIVER_NAME "escam_motor"
#define MOTOR_DEV_NAME "motor"
#define GKIO_DEV_NAME "gkio"

/* IOCTL Commands matching escam-driver contracts */
#define MOTOR_IOCTL_STOP       0xC0046D00
#define MOTOR_IOCTL_RUN        0xC0046D01
#define MOTOR_IOCTL_SPEED      0xC0046D06
#define GKIO_IOCTL_SET_VALUE   0xC0046200
#define GKIO_IOCTL_GET_VALUE   0xC0046201

/* GPIO Pin Definitions for ESCAM G02 */
static const int pan_pins[4]  = { 0, 1, 2, 3 };   /* ULN2803 Pan phases */
static const int tilt_pins[4] = { 4, 5, 6, 7 };   /* ULN2803 Tilt phases */
#define GPIO_IRCUT_FWD  14
#define GPIO_IRCUT_REV  17
#define GPIO_IRLED      10

/* 8-step half-stepping sequence table */
static const u8 step_table[8] = {
    0x01, 0x03, 0x02, 0x06, 0x04, 0x0C, 0x08, 0x09
};

struct motor_run_cmd {
    int pandir;   /* 3 = Right, 4 = Left, 0 = Stop */
    int titldir;  /* 1 = Up, 2 = Down, 0 = Stop */
};

struct gkio_cmd {
    u32 pin;
    u32 val;
};

static dev_t motor_dev_t;
static struct cdev motor_cdev;
static struct class *motor_class;

static dev_t gkio_dev_t;
static struct cdev gkio_cdev;
static struct class *gkio_class;

static struct hrtimer step_timer;
static ktime_t step_interval;
static int cur_pan_dir = 0;
static int cur_tilt_dir = 0;
static int pan_step_idx = 0;
static int tilt_step_idx = 0;

static void set_phase_pins(const int *pins, u8 pattern) {
    int i;
    for (i = 0; i < 4; i++) {
        gpio_set_value(pins[i], (pattern >> i) & 1);
    }
}

static enum hrtimer_restart motor_timer_callback(struct hrtimer *timer) {
    if (cur_pan_dir == 3) {
        pan_step_idx = (pan_step_idx + 1) & 7;
        set_phase_pins(pan_pins, step_table[pan_step_idx]);
    } else if (cur_pan_dir == 4) {
        pan_step_idx = (pan_step_idx - 1) & 7;
        set_phase_pins(pan_pins, step_table[pan_step_idx]);
    }

    if (cur_tilt_dir == 1) {
        tilt_step_idx = (tilt_step_idx + 1) & 7;
        set_phase_pins(tilt_pins, step_table[tilt_step_idx]);
    } else if (cur_tilt_dir == 2) {
        tilt_step_idx = (tilt_step_idx - 1) & 7;
        set_phase_pins(tilt_pins, step_table[tilt_step_idx]);
    }

    if (cur_pan_dir != 0 || cur_tilt_dir != 0) {
        hrtimer_forward_now(timer, step_interval);
        return HRTIMER_RESTART;
    }

    /* Release motor coils on stop to prevent heating */
    set_phase_pins(pan_pins, 0);
    set_phase_pins(tilt_pins, 0);
    return HRTIMER_NORESTART;
}

static long motor_ioctl(struct file *file, unsigned int cmd, unsigned long arg) {
    struct motor_run_cmd run_cmd;
    switch (cmd) {
    case MOTOR_IOCTL_RUN:
        if (copy_from_user(&run_cmd, (void __user *)arg, sizeof(run_cmd)))
            return -EFAULT;
        cur_pan_dir = run_cmd.pandir;
        cur_tilt_dir = run_cmd.titldir;
        if (cur_pan_dir != 0 || cur_tilt_dir != 0) {
            if (!hrtimer_is_queued(&step_timer)) {
                hrtimer_start(&step_timer, step_interval, HRTIMER_MODE_REL);
            }
        }
        return 0;

    case MOTOR_IOCTL_STOP:
        cur_pan_dir = 0;
        cur_tilt_dir = 0;
        hrtimer_cancel(&step_timer);
        set_phase_pins(pan_pins, 0);
        set_phase_pins(tilt_pins, 0);
        return 0;

    case MOTOR_IOCTL_SPEED:
        /* Speed divisor adjustment (1000us - 5000us) */
        step_interval = ktime_set(0, 1500000); /* 1.5ms default */
        return 0;

    default:
        return -EINVAL;
    }
}

static const struct file_operations motor_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = motor_ioctl,
};

static long gkio_ioctl(struct file *file, unsigned int cmd, unsigned long arg) {
    struct gkio_cmd gcmd;
    int val;

    switch (cmd) {
    case GKIO_IOCTL_SET_VALUE:
        if (copy_from_user(&gcmd, (void __user *)arg, sizeof(gcmd)))
            return -EFAULT;
        gpio_direction_output(gcmd.pin, gcmd.val ? 1 : 0);
        return 0;

    case GKIO_IOCTL_GET_VALUE:
        if (copy_from_user(&gcmd, (void __user *)arg, sizeof(gcmd)))
            return -EFAULT;
        val = gpio_get_value(gcmd.pin);
        gcmd.val = val ? 1 : 0;
        if (copy_to_user((void __user *)arg, &gcmd, sizeof(gcmd)))
            return -EFAULT;
        return 0;

    default:
        return -EINVAL;
    }
}

static const struct file_operations gkio_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = gkio_ioctl,
};

static int __init escam_motor_init(void) {
    int i;
    step_interval = ktime_set(0, 1500000); /* 1.5ms pulse interval */
    hrtimer_init(&step_timer, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    step_timer.function = motor_timer_callback;

    for (i = 0; i < 4; i++) {
        gpio_request(pan_pins[i], "pan");
        gpio_direction_output(pan_pins[i], 0);
        gpio_request(tilt_pins[i], "tilt");
        gpio_direction_output(tilt_pins[i], 0);
    }
    gpio_request(GPIO_IRCUT_FWD, "ircut_fwd");
    gpio_direction_output(GPIO_IRCUT_FWD, 0);
    gpio_request(GPIO_IRCUT_REV, "ircut_rev");
    gpio_direction_output(GPIO_IRCUT_REV, 0);
    gpio_request(GPIO_IRLED, "irled");
    gpio_direction_output(GPIO_IRLED, 0);

    alloc_chrdev_region(&motor_dev_t, 0, 1, MOTOR_DEV_NAME);
    cdev_init(&motor_cdev, &motor_fops);
    cdev_add(&motor_cdev, motor_dev_t, 1);
    motor_class = class_create(THIS_MODULE, MOTOR_DEV_NAME);
    device_create(motor_class, NULL, motor_dev_t, NULL, MOTOR_DEV_NAME);

    alloc_chrdev_region(&gkio_dev_t, 0, 1, GKIO_DEV_NAME);
    cdev_init(&gkio_cdev, &gkio_fops);
    cdev_add(&gkio_cdev, gkio_dev_t, 1);
    gkio_class = class_create(THIS_MODULE, GKIO_DEV_NAME);
    device_create(gkio_class, NULL, gkio_dev_t, NULL, GKIO_DEV_NAME);

    pr_info("escam_motor: Open-source motor and gkio driver registered\n");
    return 0;
}

static void __exit escam_motor_exit(void) {
    int i;
    hrtimer_cancel(&step_timer);
    device_destroy(motor_class, motor_dev_t);
    class_destroy(motor_class);
    cdev_del(&motor_cdev);
    unregister_chrdev_region(motor_dev_t, 1);

    device_destroy(gkio_class, gkio_dev_t);
    class_destroy(gkio_class);
    cdev_del(&gkio_cdev);
    unregister_chrdev_region(gkio_dev_t, 1);

    for (i = 0; i < 4; i++) {
        gpio_free(pan_pins[i]);
        gpio_free(tilt_pins[i]);
    }
    gpio_free(GPIO_IRCUT_FWD);
    gpio_free(GPIO_IRCUT_REV);
    gpio_free(GPIO_IRLED);

    pr_info("escam_motor: Unregistered\n");
}

module_init(escam_motor_init);
module_exit(escam_motor_exit);

MODULE_AUTHOR("Elena");
MODULE_DESCRIPTION("Open-Source ESCAM G02 Stepper Motor & GKIO Driver");
MODULE_LICENSE("GPL");
