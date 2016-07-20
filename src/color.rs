use std::ops::{Add, AddAssign, Sub, Mul, MulAssign};
use std::convert::From;

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub struct Color(pub u32);

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color((r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24))
    }

    pub fn r(&self) -> u8 {
        (self.0 & 255) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    pub fn b(&self) -> u8 {
        ((self.0 >> 16) & 255) as u8
    }

    pub fn a(&self) -> u8 {
        ((self.0 >> 24) & 255) as u8
    }
}

#[derive(Default, Copy, Clone)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl FloatColor {
    pub fn dot(&self, rhs: &FloatColor) -> f64 {
        self.r * rhs.r + self.g * rhs.g + self.b * rhs.b + self.a * rhs.a
    }

    pub fn abs(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl<'a> From<&'a Color> for FloatColor {
    fn from(c: &'a Color) -> Self {
        FloatColor {
            r: c.r() as f64 / 255.0,
            g: c.g() as f64 / 255.0,
            b: c.b() as f64 / 255.0,
            a: c.a() as f64 / 255.0,
        }
    }
}

impl From<FloatColor> for Color {
    fn from(c: FloatColor) -> Self {
        Color::rgba((c.r * 255.0).max(0.0).min(255.0) as u8,
                    (c.g * 255.0).max(0.0).min(255.0) as u8,
                    (c.b * 255.0).max(0.0).min(255.0) as u8,
                    (c.a * 255.0).max(0.0).min(255.0) as u8)
    }
}

impl Add for FloatColor {
    type Output = FloatColor;
    fn add(self, rhs: FloatColor) -> FloatColor {
        FloatColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Add<f64> for FloatColor {
    type Output = FloatColor;
    fn add(self, rhs: f64) -> FloatColor {
        FloatColor {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        }
    }
}

impl AddAssign for FloatColor {
    fn add_assign(&mut self, rhs: FloatColor) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub for FloatColor {
    type Output = FloatColor;
    fn sub(self, rhs: FloatColor) -> FloatColor {
        FloatColor {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl Mul<f64> for FloatColor {
    type Output = FloatColor;
    fn mul(self, rhs: f64) -> FloatColor {
        FloatColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl MulAssign<f64> for FloatColor {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Mul for FloatColor {
    type Output = FloatColor;
    fn mul(self, rhs: FloatColor) -> FloatColor {
        FloatColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}
