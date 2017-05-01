use std::ops::{Sub, Add, Mul};

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn mul_all(mut self, multiplier: f32) {
        self.r = (self.r as f32 * multiplier) as u8;
        self.g = (self.g as f32 * multiplier) as u8;
        self.b = (self.b as f32 * multiplier) as u8;
    }
}

impl<'a> Mul<f32> for &'a Rgb {
    type Output = Rgb;

    fn mul(self, multiplier: f32) -> Rgb {
        Rgb {
            r: (self.r as f32 * multiplier) as u8,
            g: (self.g as f32 * multiplier) as u8,
            b: (self.b as f32 * multiplier) as u8,
        }
    }
}


impl<'a, 'b> Sub<&'b Rgb> for &'a Rgb {
    type Output = Rgb;

    fn sub(self, other: &'b Rgb) -> Rgb {
        Rgb {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl<'a, 'b> Add<&'b Rgb> for &'a Rgb {
    type Output = Rgb;

    fn add(self, other: &'b Rgb) -> Rgb {
        Rgb {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Rgb) -> bool {
        self.r == other.r  && self.g == other.g && self.b == other.b
    }
}