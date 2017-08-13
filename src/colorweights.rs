use Colorf;

pub trait Weighted {
    fn to_weighted(&self, color: Colorf) -> Colorf;
    fn from_weighted(&self, color: Colorf) -> Colorf;
}

pub struct ColorWeights {
    pub transparency_scale: f64,
    pub weights: Colorf,
}

impl Weighted for ColorWeights {
    fn to_weighted(&self, color: Colorf) -> Colorf {
        let mut color = color * self.weights;
        let f = color.a * (1.0 - self.transparency_scale) + self.transparency_scale;
        color.r *= f;
        color.g *= f;
        color.b *= f;
        color
    }

    fn from_weighted(&self, mut color: Colorf) -> Colorf {
        let f = 1. / (color.a * (1.0 - self.transparency_scale) + self.transparency_scale);
        color.r *= f;
        color.g *= f;
        color.b *= f;
        color / self.color
    }
}

impl Default for ColorWeights {
    fn default() -> ColorWeights {
        ColorWeights {
            transparency_scale: 0.01,
            weights: Colorf {
                r: 1.0,
                g: 1.2,
                b: 0.8,
                a: 0.75,
            },
        }
    }
}
