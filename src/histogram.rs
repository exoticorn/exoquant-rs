use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};

use super::*;
use super::quantizer::HistColor;

pub struct Histogram {
    data: HashMap<Color, usize>,
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram { data: HashMap::new() }
    }

    pub fn to_hist_colors<T: ColorSpace>(&self, colorspace: &T) -> Vec<HistColor> {
        self.data
            .iter()
            .map(|(color, count)| {
                HistColor {
                    color: colorspace.to_float(*color),
                    count: *count,
                }
            })
            .collect()
    }
}

impl Extend<Color> for Histogram {
    fn extend<T>(&mut self, iter: T)
        where T: IntoIterator<Item = Color>
    {
        for pixel in iter {
            let count = self.data.entry(pixel).or_insert(0);
            *count += 1;
        }
    }
}

impl FromIterator<Color> for Histogram {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = Color>
    {
        let mut histogram = Histogram::new();
        histogram.extend(iter.into_iter());
        histogram
    }
}

#[cfg(test)]
mod tests {
    use ::Color;

    #[test]
    fn count_duplicates() {
        let mut hist = super::Histogram::new();
        hist.extend([0xaabbccffu32, 0x00ff00ff, 0x330088ff, 0x00ff00ff]
            .iter()
            .map(|c| Color(*c)));
        assert_eq!(*hist.data.get(&Color(0x00ff00ff)).unwrap(), 2usize);
    }
}
