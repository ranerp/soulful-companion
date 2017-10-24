extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

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

