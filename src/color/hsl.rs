use std::ops::{Sub, Add, Mul};

#[derive(Debug)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    pub fn new(h: f32, s:f32, l:f32) -> Hsl {
        Hsl {
            h: h,
            s: s,
            l: l,
        }
    }

    pub fn mul_all(mut self, multiplier: f32) {
        self.h = self.h * multiplier;
        self.s = self.s * multiplier;
        self.l = self.l * multiplier;
    }
}

impl Mul<f32> for Hsl {
    type Output = Hsl;

    fn mul(self, multiplier: f32) -> Hsl {
        Hsl {
            h: self.h * multiplier,
            s: self.s * multiplier,
            l: self.l * multiplier,
        }
    }
}

impl<'a, 'b> Sub<&'b Hsl> for &'a Hsl {
    type Output = Hsl;

    fn sub(self, other: &'b Hsl) -> Hsl {
        Hsl {
            h: self.h - other.h,
            s: self.s - other.s,
            l: self.l - other.l,
        }
    }
}

impl<'a, 'b> Add<&'b Hsl> for &'a Hsl {
    type Output = Hsl;

    fn add(self, other: &'b Hsl) -> Hsl {
        Hsl {
            h: self.h + other.h,
            s: self.s + other.s,
            l: self.l + other.l,
        }
    }
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Hsl) -> bool {
        self.h == other.h  && self.s == other.s && self.l == other.l
    }
}