extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const ADDR: u16 = 0x40;

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller {

        }
    }

    pub fn test(&self) {
        let mut dev = LinuxI2CDevice::new("/dev/i2c-1", ADDR).unwrap();

        println!("testing controller");
    }
}