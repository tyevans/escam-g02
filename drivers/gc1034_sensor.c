/*
 * GalaxyCore GC1034 CMOS Image Sensor Open-Source Linux Driver
 *
 * Implements a standard V4L2 subdevice I2C driver for the GC1034.
 * Replaces proprietary vendor modules: gc1034_ex.ko and sensor.ko.
 * Target SoC: Goke GK7102C (ARMv6) / Mainline Linux
 * License: GPL-2.0
 */

#include <linux/module.h>
#include <linux/init.h>
#include <linux/i2c.h>
#include <linux/delay.h>
#include <linux/videodev2.h>
#include <media/v4l2-subdev.h>
#include <media/v4l2-ctrls.h>
#include <media/v4l2-device.h>

#define GC1034_NAME "gc1034"
#define GC1034_I2C_ADDR 0x21

/* GC1034 Register Map */
#define GC1034_REG_CHIP_ID_H      0xF0
#define GC1034_REG_CHIP_ID_L      0xF1
#define GC1034_CHIP_ID            0x1034

#define GC1034_REG_EXP_H          0x03
#define GC1034_REG_EXP_L          0x04
#define GC1034_REG_ANALOG_GAIN    0xB6

struct gc1034_dev {
    struct i2c_client *client;
    struct v4l2_subdev sd;
    struct v4l2_ctrl_handler ctrl_handler;
    struct v4l2_ctrl *exposure;
    struct v4l2_ctrl *gain;
    struct mutex lock;
    bool streaming;
};

static inline struct gc1034_dev *to_gc1034(struct v4l2_subdev *sd) {
    return container_of(sd, struct gc1034_dev, sd);
}

static int gc1034_read_reg(struct i2c_client *client, u8 reg, u8 *val) {
    int ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0) {
        dev_err(&client->dev, "I2C read reg 0x%02x failed: %d\n", reg, ret);
        return ret;
    }
    *val = (u8)ret;
    return 0;
}

static int gc1034_write_reg(struct i2c_client *client, u8 reg, u8 val) {
    int ret = i2c_smbus_write_byte_data(client, reg, val);
    if (ret < 0) {
        dev_err(&client->dev, "I2C write reg 0x%02x failed: %d\n", reg, ret);
        return ret;
    }
    return 0;
}

static int gc1034_set_exposure(struct gc1034_dev *sensor, s32 val) {
    struct i2c_client *client = sensor->client;
    u8 exp_h = (val >> 8) & 0x3F;
    u8 exp_l = val & 0xFF;
    int ret;

    ret = gc1034_write_reg(client, GC1034_REG_EXP_H, exp_h);
    if (ret) return ret;
    return gc1034_write_reg(client, GC1034_REG_EXP_L, exp_l);
}

static int gc1034_set_gain(struct gc1034_dev *sensor, s32 val) {
    struct i2c_client *client = sensor->client;
    return gc1034_write_reg(client, GC1034_REG_ANALOG_GAIN, (u8)(val & 0xFF));
}

static int gc1034_s_ctrl(struct v4l2_ctrl *ctrl) {
    struct gc1034_dev *sensor = container_of(ctrl->handler, struct gc1034_dev, ctrl_handler);

    switch (ctrl->id) {
    case V4L2_CID_EXPOSURE:
        return gc1034_set_exposure(sensor, ctrl->val);
    case V4L2_CID_GAIN:
        return gc1034_set_gain(sensor, ctrl->val);
    default:
        return -EINVAL;
    }
}

static const struct v4l2_ctrl_ops gc1034_ctrl_ops = {
    .s_ctrl = gc1034_s_ctrl,
};

static int gc1034_s_stream(struct v4l2_subdev *sd, int enable) {
    struct gc1034_dev *sensor = to_gc1034(sd);
    mutex_lock(&sensor->lock);
    sensor->streaming = !!enable;
    mutex_unlock(&sensor->lock);
    return 0;
}

static const struct v4l2_subdev_video_ops gc1034_video_ops = {
    .s_stream = gc1034_s_stream,
};

static const struct v4l2_subdev_ops gc1034_subdev_ops = {
    .video = &gc1034_video_ops,
};

static int gc1034_check_chip_id(struct i2c_client *client) {
    u8 id_h, id_l;
    u16 chip_id;
    int ret;

    ret = gc1034_read_reg(client, GC1034_REG_CHIP_ID_H, &id_h);
    if (ret) return ret;
    ret = gc1034_read_reg(client, GC1034_REG_CHIP_ID_L, &id_l);
    if (ret) return ret;

    chip_id = ((u16)id_h << 8) | id_l;
    if (chip_id != GC1034_CHIP_ID) {
        dev_err(&client->dev, "Unexpected Chip ID: 0x%04x (expected 0x1034)\n", chip_id);
        return -ENODEV;
    }

    dev_info(&client->dev, "Found GalaxyCore GC1034 sensor (ID: 0x%04x)\n", chip_id);
    return 0;
}

static int gc1034_probe(struct i2c_client *client, const struct i2c_device_id *id) {
    struct gc1034_dev *sensor;
    int ret;

    ret = gc1034_check_chip_id(client);
    if (ret) return ret;

    sensor = devm_kzalloc(&client->dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor) return -ENOMEM;

    sensor->client = client;
    mutex_init(&sensor->lock);
    v4l2_i2c_subdev_init(&sensor->sd, client, &gc1034_subdev_ops);

    v4l2_ctrl_handler_init(&sensor->ctrl_handler, 2);
    sensor->exposure = v4l2_ctrl_new_std(&sensor->ctrl_handler, &gc1034_ctrl_ops,
                                         V4L2_CID_EXPOSURE, 1, 4000, 1, 400);
    sensor->gain = v4l2_ctrl_new_std(&sensor->ctrl_handler, &gc1034_ctrl_ops,
                                     V4L2_CID_GAIN, 0, 255, 1, 0);

    if (sensor->ctrl_handler.error) {
        ret = sensor->ctrl_handler.error;
        v4l2_ctrl_handler_free(&sensor->ctrl_handler);
        return ret;
    }

    sensor->sd.ctrl_handler = &sensor->ctrl_handler;
    v4l2_info(&sensor->sd, "GC1034 subdevice probed successfully\n");
    return 0;
}

static int gc1034_remove(struct i2c_client *client) {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct gc1034_dev *sensor = to_gc1034(sd);

    v4l2_device_unregister_subdev(&sensor->sd);
    v4l2_ctrl_handler_free(&sensor->ctrl_handler);
    mutex_destroy(&sensor->lock);
    return 0;
}

static const struct i2c_device_id gc1034_id[] = {
    { "gc1034", 0 },
    { }
};
MODULE_DEVICE_TABLE(i2c, gc1034_id);

static struct i2c_driver gc1034_i2c_driver = {
    .driver = {
        .name = GC1034_NAME,
    },
    .probe    = gc1034_probe,
    .remove   = gc1034_remove,
    .id_table = gc1034_id,
};

module_i2c_driver(gc1034_i2c_driver);

MODULE_AUTHOR("Elena");
MODULE_DESCRIPTION("Open-Source GalaxyCore GC1034 Image Sensor V4L2 Driver");
MODULE_LICENSE("GPL");
