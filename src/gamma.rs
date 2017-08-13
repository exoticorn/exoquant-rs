use Colorf;

pub const QUANTIZATION_GAMMA: f64 = 2.0;

pub trait Gamma {
    fn from_linear(&self, color: Colorf) -> Colorf;
    fn to_linear(&self, color: Colorf) -> Colorf;

    fn to_quantization(&self, color: Colorf) -> Colorf {
        QUANTIZATION_GAMMA.from_linear(self.to_linear(color))
    }

    fn from_quantization(&self, color: Colorf) -> Colorf {
        self.from_linear(QUANTIZATION_GAMMA.to_linear(color))
    }
}

impl Gamma for f64 {
    fn from_linear(&self, color: Colorf) -> Colorf {
        color.pow(*self)
    }

    fn to_linear(&self, color: Colorf) -> Colorf {
        color.pow(1.0 / *self)
    }
}
