extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const MAG_ADDRESS: u8 = 0x1E;
const ACC_ADDRESS: u8 = 0x1E;
const GYR_ADDRESS: u8 = 0x6A;

//LSM9DS0 Gyro Registers
const WHO_AM_I_G: u8 = 0x0F;
const CTRL_REG1_G: u8 = 0x20;
const CTRL_REG2_G: u8 = 0x21;
const CTRL_REG3_G: u8 = 0x22;
const CTRL_REG4_G: u8 = 0x23;
const CTRL_REG5_G: u8 = 0x24;
const REFERENCE_G: u8 = 0x25;
const STATUS_REG_G: u8 = 0x27;
const OUT_X_L_G: u8 = 0x28;
const OUT_X_H_G: u8 = 0x29;
const OUT_Y_L_G: u8 = 0x2A;
const OUT_Y_H_G: u8 = 0x2B;
const OUT_Z_L_G: u8 = 0x2C;
const OUT_Z_H_G: u8 = 0x2D;
const FIFO_CTRL_REG_G: u8 = 0x2E;
const FIFO_SRC_REG_G: u8 = 0x2F;
const INT1_CFG_G: u8 = 0x30;
const INT1_SRC_G: u8 = 0x31;
const INT1_THS_XH_G: u8 = 0x32;
const INT1_THS_XL_G: u8 = 0x33;
const INT1_THS_YH_G: u8 = 0x34;
const INT1_THS_YL_G: u8 = 0x35;
const INT1_THS_ZH_G: u8 = 0x36;
const  INT1_THS_ZL_G: u8 = 0x37;
const INT1_DURATION_G: u8 = 0x38;

//LSM9DS0 Accel and Magneto Registers
const OUT_TEMP_L_XM: u8 = 0x05;
const OUT_TEMP_H_XM: u8 = 0x06;
const STATUS_REG_M: u8 = 0x07;
const OUT_X_L_M: u8 = 0x08;
const OUT_X_H_M: u8 = 0x09;
const OUT_Y_L_M: u8 = 0x0A;
const OUT_Y_H_M: u8 = 0x0B;
const OUT_Z_L_M: u8 = 0x0C;
const OUT_Z_H_M: u8 = 0x0D;
const WHO_AM_I_XM: u8 = 0x0F;
const INT_CTRL_REG_M: u8 = 0x12;
const INT_SRC_REG_M: u8 = 0x13;
const INT_THS_L_M: u8 = 0x14;
const INT_THS_H_M: u8 = 0x15;
const OFFSET_X_L_M: u8 = 0x16;
const OFFSET_X_H_M: u8 = 0x17;
const OFFSET_Y_L_M: u8 = 0x18;
const OFFSET_Y_H_M: u8 = 0x19;
const OFFSET_Z_L_M: u8 = 0x1A;
const OFFSET_Z_H_M: u8 = 0x1B;
const REFERENCE_X: u8 = 0x1C;
const REFERENCE_Y: u8 = 0x1D;
const REFERENCE_Z: u8 = 0x1E;
const CTRL_REG0_XM: u8 = 0x1F;
const CTRL_REG1_XM: u8 = 0x20;
const CTRL_REG2_XM: u8 = 0x21;
const CTRL_REG3_XM: u8 = 0x22;
const CTRL_REG4_XM: u8 = 0x23;
const CTRL_REG5_XM: u8 = 0x24;
const CTRL_REG6_XM: u8 = 0x25;
const CTRL_REG7_XM: u8 = 0x26;
const STATUS_REG_A: u8 = 0x27;
const OUT_X_L_A: u8 = 0x28;
const OUT_X_H_A: u8 = 0x29;
const OUT_Y_L_A: u8 = 0x2A;
const OUT_Y_H_A: u8 = 0x2B;
const OUT_Z_L_A: u8 = 0x2C;
const OUT_Z_H_A: u8 = 0x2D;
const FIFO_CTRL_REG: u8 = 0x2E;
const FIFO_SRC_REG: u8 = 0x2F;
const INT_GEN_1_REG: u8 = 0x30;
const INT_GEN_1_SRC: u8 = 0x31;
const INT_GEN_1_THS: u8 = 0x32;
const INT_GEN_1_DURATION: u8 = 0x33;
const INT_GEN_2_REG: u8 = 0x34;
const INT_GEN_2_SRC: u8 = 0x35;
const INT_GEN_2_THS: u8 = 0x36;
const INT_GEN_2_DURATION: u8 = 0x37;
const CLICK_CFG: u8 = 0x38;
const CLICK_SRC: u8 = 0x39;
const CLICK_THS: u8 = 0x3A;
const TIME_LIMIT: u8 = 0x3B;
const TIME_LATENCY: u8 = 0x3C;
const TIME_WINDOW: u8 = 0x3D;

pub struct BerryIMUController {
    dev: LinuxI2CDevice
}

impl BerryIMUController {
    fn new() -> BerryIMUController {
        BerryIMUController {
            dev: LinuxI2CDevice::new("/dev/i2c-1", PCA9685_ADDR as u16).unwrap()
        }
    }
}

