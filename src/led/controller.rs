extern crate i2cdev;

use std::time::Duration;
use std::thread;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

// Registers/etc:
const PCA9685_ADDR: u8 = 0x40;
const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
const SUBADR1: u8 = 0x02;
const SUBADR2: u8 = 0x03;
const SUBADR3: u8 = 0x04;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_LED_ON_L: u8 = 0xFA;
const ALL_LED_ON_H: u8 = 0xFB;
const ALL_LED_OFF_L: u8 = 0xFC;
const ALL_LED_OFF_H: u8 = 0xFD;

// Bits:
const RESTART: u8 = 0x80;
const SLEEP: u8 = 0x10;
const ALLCALL: u8 = 0x01;
const INVRT: u8 = 0x10;
const OUTDRV: u8 = 0x04;

pub struct Controller {
    dev: LinuxI2CDevice
}

impl Controller {
    pub fn new() -> Controller {
        let mut controller = Controller {
            dev: LinuxI2CDevice::new("/dev/i2c-1", PCA9685_ADDR as u16).unwrap()
        };

        controller.init();

        controller
    }

    fn init(&mut self) {
        self.set_all_pwm(0, 0);

        self.dev.smbus_write_byte_data(MODE2, OUTDRV).unwrap();
        self.dev.smbus_write_byte_data(MODE1, ALLCALL).unwrap();

        thread::sleep(Duration::from_millis(5));

        let mut mode1 = self.dev.smbus_read_byte_data(MODE1).unwrap();
        mode1 = mode1 & !SLEEP; // wake up (reset sleep)

        self.dev.smbus_write_byte_data(MODE1, mode1).unwrap();

        thread::sleep(Duration::from_millis(5));
    }

    pub fn software_restart(&mut self) {
        self.dev.smbus_write_byte(0x06).unwrap(); // Software restart
    }

    pub fn set_pwm_freq(&mut self, freq_hz: f64) {
        let mut prescale_val = 25_000_000.0; // 25MHz
        prescale_val /= 4096.0; // 12-bit
        prescale_val /= freq_hz;
        prescale_val -= 1.0;

        println!("Setting PWM frequency to {} Hz", freq_hz);
        println!("Estimated pre-scale: {}", prescale_val);

        let prescale_val = (prescale_val + 0.5).floor() as u8;

        println!("Final pre-scale: {}", prescale_val);

        let old_mode = self.dev.smbus_read_byte_data(MODE1).unwrap();
        let new_mode = (old_mode & 0x7F) | 0x10; /// sleep

        self.dev.smbus_write_byte_data(MODE1, new_mode).unwrap(); /// go to sleep
        self.dev.smbus_write_byte_data(PRESCALE, prescale_val).unwrap();
        self.dev.smbus_write_byte_data(MODE1, old_mode).unwrap();

        thread::sleep(Duration::from_millis(5));

        self.dev.smbus_write_byte_data(MODE1, old_mode | 0x80).unwrap();
    }

    // TODO Find out proper shifting without exceeding the type's number of bits and fix it!
    pub fn set_pwm(&mut self, channel: u8, on: u8, off: u8) {
        self.dev.smbus_write_byte_data(LED0_ON_L + 4 * channel, on & 0xFF).unwrap();
        self.dev.smbus_write_byte_data(LED0_ON_H + 4 * channel, on >> 8).unwrap();
        self.dev.smbus_write_byte_data(LED0_OFF_L + 4 * channel, off & 0xFF).unwrap();
        self.dev.smbus_write_byte_data(LED0_OFF_H + 4 * channel, off >> 7).unwrap();
    }

    // TODO Find out proper shifting without exceeding the type's number of bits and fix it!
    fn set_all_pwm(&mut self, on: u8, off: u8) {
        self.dev.smbus_write_byte_data(ALL_LED_ON_L, on & 0xFF).unwrap();
        self.dev.smbus_write_byte_data(ALL_LED_ON_H, on >> 7).unwrap();
        self.dev.smbus_write_byte_data(ALL_LED_OFF_L, off & 0xFF).unwrap();
        self.dev.smbus_write_byte_data(ALL_LED_OFF_H, off >> 7).unwrap();
    }
}