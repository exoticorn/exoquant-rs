use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div};

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Colorf {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Colorf {
    pub fn zero() -> Colorf {
        Colorf {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
        }
    }

    pub fn dot(&self, rhs: &Colorf) -> f64 {
        self.r * rhs.r + self.g * rhs.g + self.b * rhs.b + self.a * rhs.a
    }

    pub fn abs(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn pow(&self, e: f64) -> Colorf {
        Colorf {
            r: self.r.max(0.0).powf(e),
            g: self.g.max(0.0).powf(e),
            b: self.b.max(0.0).powf(e),
            a: self.a.max(0.0).powf(e),
        }
    }
}

impl Add for Colorf {
    type Output = Colorf;
    fn add(self, rhs: Colorf) -> Colorf {
        Colorf {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Add<f64> for Colorf {
    type Output = Colorf;
    fn add(self, rhs: f64) -> Colorf {
        Colorf {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        }
    }
}

impl AddAssign for Colorf {
    fn add_assign(&mut self, rhs: Colorf) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub for Colorf {
    type Output = Colorf;
    fn sub(self, rhs: Colorf) -> Colorf {
        Colorf {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl Mul<f64> for Colorf {
    type Output = Colorf;
    fn mul(self, rhs: f64) -> Colorf {
        Colorf {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl MulAssign<f64> for Colorf {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Mul for Colorf {
    type Output = Colorf;
    fn mul(self, rhs: Colorf) -> Colorf {
        Colorf {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Div for Colorf {
    type Output = Colorf;
    fn div(self, rhs: Colorf) -> Colorf {
        Colorf {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}
