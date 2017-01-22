use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div};

/// A RGBA8 color used for both the input image data and the palette output.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Creates a new `Color` from the given channel components.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

/// A color with floating point channel components.
///
/// Used for all internal processing.
///
/// It implements `Mul`, `Div`, `Add` and `Sub` operators that apply each operation component
/// wise to each channel in turn.
#[derive(Default, Copy, Clone, Debug)]
pub struct Colorf {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Colorf {
    /// Returns a `Colorf` with all channel components set to zero.
    pub fn zero() -> Colorf {
        Colorf {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
        }
    }

    /// Returns the dot product of two `Colorf`s.
    pub fn dot(&self, rhs: &Colorf) -> f64 {
        self.r * rhs.r + self.g * rhs.g + self.b * rhs.b + self.a * rhs.a
    }

    /// Returns the magnitude (vector length) of a `Colorf`.
    pub fn abs(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns a new `Colorf` with each color component raised to the given power.
    pub fn pow(&self, e: f64) -> Colorf {
        Colorf {
            r: self.r.max(0.0).powf(e),
            g: self.g.max(0.0).powf(e),
            b: self.b.max(0.0).powf(e),
            a: self.a.max(0.0).powf(e),
        }
    }
}

impl From<Color> for Colorf {
    fn from(c: Color) -> Colorf {
        Colorf {
            r: c.r as f64 / 255.,
            g: c.g as f64 / 255.,
            b: c.b as f64 / 255.,
            a: c.a as f64 / 255.,
        }
    }
}

impl From<Colorf> for Color {
    fn from(c: Colorf) -> Color {
        Color::new((c.r * 255.).max(0.).min(255.) as u8,
                   (c.g * 255.).max(0.).min(255.) as u8,
                   (c.b * 255.).max(0.).min(255.) as u8,
                   (c.a * 255.).max(0.).min(255.) as u8)
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
