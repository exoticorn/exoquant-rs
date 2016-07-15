use ::color::Color;
use ::color::FloatColor;

pub struct ColorMap {
    colors: Vec<FloatColor>,
}

impl ColorMap {
    pub fn new(colors: &[Color]) -> ColorMap {
        ColorMap { colors: colors.iter().map(|c| c.into()).collect() }
    }

    pub fn find_nearest(&self, color: &Color) -> usize {
        let c: FloatColor = color.into();
        let mut best_index = 0;
        let mut best_error = ::std::f64::MAX;
        for i in 0..self.colors.len() {
            let diff = self.colors[i] - c;
            let e = diff.dot(&diff);
            if e < best_error {
                best_error = e;
                best_index = i;
            }
        }
        best_index
    }
}
