use super::*;

/// Defines the colorspaces in which to do quantization and remapping
///
/// The current implementation is subject to change. Just use
/// `SimpleColorSpace::default()` for now wherever a ColorSpace parameter
/// is required..

pub trait ColorSpace {
    fn input_to_linear(&self, color: Colorf) -> Colorf;

    fn target_gamma(&self) -> f64;

    fn to_scaled(&self, color: Colorf) -> Colorf;
    fn from_scaled(&self, color: Colorf) -> Colorf;

    fn input_to_quantization(&self, color: Colorf) -> Colorf {
        let color = self.input_to_linear(color);
        let color = self.to_target_gamma(color);
        self.to_scaled(color)
    }

    fn quantization_to_output(&self, color: Colorf) -> Colorf {
        let color = self.from_scaled(color);
        let color = self.from_target_gamma(color);
        self.linear_to_output(color)
    }

    fn quantization_to_dither(&self, color: Colorf) -> Colorf {
        let color = self.from_scaled(color);
        let color = self.from_target_gamma(color);
        self.to_scaled(color)
    }

    fn dither_to_quantization(&self, color: Colorf) -> Colorf {
        let color = self.from_scaled(color);
        let color = self.to_target_gamma(color);
        self.to_scaled(color)
    }

    fn output_to_quantization(&self, color: Colorf) -> Colorf {
        let color = self.output_to_linear(color);
        let color = self.to_target_gamma(color);
        self.to_scaled(color)
    }

    fn linear_to_output(&self, color: Colorf) -> Colorf {
        color.pow(1. / self.target_gamma())
    }

    fn output_to_linear(&self, color: Colorf) -> Colorf {
        color.pow(self.target_gamma())
    }

    fn to_target_gamma(&self, color: Colorf) -> Colorf {
        color.pow(1. / self.target_gamma())
    }

    fn from_target_gamma(&self, color: Colorf) -> Colorf {
        color.pow(self.target_gamma())
    }
}

/// The default colorspace implementation.

pub struct SimpleColorSpace {
    pub source_gamma: f64,
    pub target_gamma: f64,
    pub transparency_scale: f64,
    pub scale: Colorf,
}

impl Default for SimpleColorSpace {
    fn default() -> SimpleColorSpace {
        SimpleColorSpace {
            source_gamma: 2.2,
            target_gamma: 2.2,
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
    fn input_to_linear(&self, color: Colorf) -> Colorf {
        color.pow(self.source_gamma)
    }

    fn target_gamma(&self) -> f64 {
        self.target_gamma
    }

    fn to_scaled(&self, color: Colorf) -> Colorf {
        let mut color = color * self.scale;
        let f = color.a * (1.0 - self.transparency_scale) + self.transparency_scale;
        color.r *= f;
        color.g *= f;
        color.b *= f;
        color
    }

    fn from_scaled(&self, mut color: Colorf) -> Colorf {
        let f = 1. / (color.a * (1.0 - self.transparency_scale) + self.transparency_scale);
        color.r *= f;
        color.g *= f;
        color.b *= f;
        color / self.scale
    }
}
