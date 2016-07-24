use ::color::Color;
use ::color::FloatColor;

pub trait ColorSpace {
    fn to_linear(&self, color: FloatColor) -> FloatColor;
    fn from_linear(&self, color: FloatColor) -> FloatColor;

    fn to_float(&self, c: Color) -> FloatColor {
        self.to_linear(FloatColor {
            r: c.r() as f64 / 255.0,
            g: c.g() as f64 / 255.0,
            b: c.b() as f64 / 255.0,
            a: c.a() as f64 / 255.0,
        })
    }
    fn from_float(&self, c: FloatColor) -> Color {
        let c = self.from_linear(c);
        Color::rgba((c.r * 255.0).max(0.0).min(255.0) as u8,
                    (c.g * 255.0).max(0.0).min(255.0) as u8,
                    (c.b * 255.0).max(0.0).min(255.0) as u8,
                    (c.a * 255.0).max(0.0).min(255.0) as u8)
    }
}

pub struct SimpleColorSpace {
    gamma: f64,
    scale: FloatColor,
}

impl SimpleColorSpace {
    pub fn default() -> SimpleColorSpace {
        SimpleColorSpace {
            gamma: 1.145,
            scale: FloatColor {
                r: 1.0,
                g: 1.2,
                b: 0.8,
                a: 0.75,
            },
        }
    }
}

impl ColorSpace for SimpleColorSpace {
    fn to_linear(&self, color: FloatColor) -> FloatColor {
        FloatColor {
            r: color.r.powf(self.gamma),
            g: color.g.powf(self.gamma),
            b: color.b.powf(self.gamma),
            a: color.a.powf(self.gamma),
        } * self.scale
    }

    fn from_linear(&self, color: FloatColor) -> FloatColor {
        let c = color / self.scale;
        let g = 1.0 / self.gamma;
        FloatColor {
            r: c.r.powf(g),
            g: c.g.powf(g),
            b: c.b.powf(g),
            a: c.a.powf(g),
        }
    }
}
