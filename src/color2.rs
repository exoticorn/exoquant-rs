use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy)]
pub struct SrgbColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl SrgbColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> SrgbColor {
        SrgbColor { r, g, b, a }
    }
}

fn srgb_to_linear(v: f32) -> f32 {
    if v < 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(v: f32) -> f32 {
    if v < 0.00313 {
        v * 12.92
    } else {
        v.powf(1. / 2.4) * 1.055 - 0.055
    }
}

pub trait VectorColor {
    fn to_vec4(self) -> Vec4<Self>;
    fn from_vec4(v: Vec4<Self>) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct XyzColor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

impl VectorColor for XyzColor {
    fn to_vec4(self) -> Vec4<Self> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.a,
            t: PhantomData,
        }
    }
    fn from_vec4(v: Vec4<Self>) -> Self {
        XyzColor {
            x: v.x,
            y: v.y,
            z: v.z,
            a: v.w,
        }
    }
}

impl From<SrgbColor> for XyzColor {
    fn from(other: SrgbColor) -> XyzColor {
        let r = srgb_to_linear(other.r);
        let g = srgb_to_linear(other.g);
        let b = srgb_to_linear(other.b);
        XyzColor {
            x: r * 0.4124 + g * 0.3576 + b * 0.1805,
            y: r * 0.2126 + g * 0.7152 + b * 0.0722,
            z: r * 0.0193 + g * 0.1192 + b * 0.9505,
            a: other.a,
        }
    }
}

impl Into<SrgbColor> for XyzColor {
    fn into(self) -> SrgbColor {
        let r = self.x * 3.2406 + self.y * -1.5372 + self.z * -0.4986;
        let g = self.x * -0.9689 + self.y * 1.8758 + self.z * 0.0415;
        let b = self.x * 0.0557 + self.y * -0.204 + self.z * 1.057;
        SrgbColor {
            r: linear_to_srgb(r),
            g: linear_to_srgb(g),
            b: linear_to_srgb(b),
            a: self.a,
        }
    }
}

fn linear_to_lab(v: f32) -> f32 {
    if v < 0.008856 {
        v * 7.787 + 16. / 116.
    } else {
        v.powf(0.33333)
    }
}

fn lab_to_linear(v: f32) -> f32 {
    if v < 0.20689 {
        (v - 16. / 116.) / 7.787
    } else {
        v.powf(3.)
    }
}

impl Into<LabColor> for XyzColor {
    fn into(self) -> LabColor {
        let x = linear_to_lab(self.x / 0.95047);
        let y = linear_to_lab(self.y);
        let z = linear_to_lab(self.z / 1.08883);
        let l = y * 116. - 16.;
        let a = 500. * (x - y);
        let b = 200. * (y - z);
        let f = self.a + 0.001;
        LabColor {
            l: l * f,
            a: a * f,
            b: b * f,
            alpha: self.a,
        }
    }
}

impl From<LabColor> for XyzColor {
    fn from(other: LabColor) -> XyzColor {
        let f = 1. / (other.alpha + 0.001);
        let y = (other.l * f + 16.) / 116.;
        let x = other.a * f / 500. + y;
        let z = y - other.b * f / 200.;
        let x = lab_to_linear(x);
        let y = lab_to_linear(y);
        let z = lab_to_linear(z);
        XyzColor {
            x: x * 0.95047,
            y,
            z: z * 1.08883,
            a: other.alpha,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LabColor {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T: ?Sized> {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    t: PhantomData<T>,
}

impl<T: Copy> Vec4<T> {
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    pub fn abs(self) -> f32 {
        self.dot(self).sqrt()
    }
}

impl<T> Add for Vec4<T> {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Vec4 {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
            w: self.w + o.w,
            t: PhantomData,
        }
    }
}

impl<T> AddAssign for Vec4<T> {
    fn add_assign(&mut self, o: Self) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
        self.w += o.w;
    }
}

impl<T> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Vec4 {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
            w: self.w - o.w,
            t: PhantomData,
        }
    }
}

impl<T> Mul for Vec4<T> {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Vec4 {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
            w: self.w * o.w,
            t: PhantomData,
        }
    }
}

impl<T> MulAssign for Vec4<T> {
    fn mul_assign(&mut self, o: Self) {
        self.x *= o.x;
        self.y *= o.y;
        self.z *= o.z;
        self.w *= o.w;
    }
}

impl<T> Div for Vec4<T> {
    type Output = Self;

    fn div(self, o: Self) -> Self {
        Vec4 {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
            w: self.w / o.w,
            t: PhantomData,
        }
    }
}
