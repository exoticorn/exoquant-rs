use super::*;

pub trait ColorSpace {
    fn to_linear(&self, color: Colorf) -> Colorf;
    fn from_linear(&self, color: Colorf) -> Colorf;

    fn to_dither(&self, c: Colorf) -> Colorf {
        c
    }
    fn from_dither(&self, c: Colorf) -> Colorf {
        c
    }

    fn to_float(&self, c: Color) -> Colorf {
        self.to_linear(Colorf {
            r: c.r as f64 / 255.0,
            g: c.g as f64 / 255.0,
            b: c.b as f64 / 255.0,
            a: c.a as f64 / 255.0,
        })
    }
    fn from_float(&self, c: Colorf) -> Color {
        let c = self.from_linear(c);
        Color::new((c.r * 255.0).max(0.0).min(255.0) as u8,
                   (c.g * 255.0).max(0.0).min(255.0) as u8,
                   (c.b * 255.0).max(0.0).min(255.0) as u8,
                   (c.a * 255.0).max(0.0).min(255.0) as u8)
    }
}

pub struct SimpleColorSpace {
    pub gamma: f64,
    pub dither_gamma: f64,
    pub transparency_scale: f64,
    pub scale: Colorf,
}

impl Default for SimpleColorSpace {
    fn default() -> SimpleColorSpace {
        SimpleColorSpace {
            gamma: 1.145,
            dither_gamma: 2.2,
            transparency_scale: 0.01,
            scale: Colorf {
                r: 1.0,
                g: 1.2,
                b: 0.8,
                a: 0.75,
            },
        }
    }
}

impl ColorSpace for SimpleColorSpace {
    fn to_linear(&self, color: Colorf) -> Colorf {
        let mut color = color.pow(self.gamma) * self.scale;
        let f = color.a * (1.0 - self.transparency_scale) + self.transparency_scale;
        color.r *= f;
        color.g *= f;
        color.b *= f;
        color
    }

    fn from_linear(&self, color: Colorf) -> Colorf {
        let c = color / self.scale;
        let g = 1.0 / self.gamma;
        let mut c = c.pow(g);
        let f = 1.0 / (color.a * (1.0 - self.transparency_scale) + self.transparency_scale);
        c.r *= f;
        c.g *= f;
        c.b *= f;
        c
    }

    fn to_dither(&self, color: Colorf) -> Colorf {
        color.pow(self.dither_gamma / self.gamma)
    }

    fn from_dither(&self, color: Colorf) -> Colorf {
        color.pow(self.gamma / self.dither_gamma)
    }
}
