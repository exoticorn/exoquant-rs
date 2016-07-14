use std::ops::Add;
use std::convert::From;

#[derive(Eq,PartialEq,Hash)]
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

pub struct FloatColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for FloatColor {
    fn from(c: Color) -> Self {
        FloatColor {
            r: c.r() as f32 / 255.0,
            g: c.g() as f32 / 255.0,
            b: c.b() as f32 / 255.0,
            a: c.a() as f32 / 255.0,
        }
    }
}

impl<'a> Add for &'a FloatColor {
    type Output = FloatColor;
    fn add(self, rhs: &FloatColor) -> FloatColor {
        FloatColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}
