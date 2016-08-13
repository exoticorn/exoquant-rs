use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};

use super::*;

pub struct Histogram {
    data: HashMap<Color, usize>,
}

#[derive(Clone)]
pub struct ColorCount {
    pub color: Colorf,
    pub count: usize,
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram { data: HashMap::new() }
    }

    pub fn to_color_counts<T: ColorSpace>(&self, colorspace: &T) -> Vec<ColorCount> {
        self.data
            .iter()
            .map(|(color, count)| {
                ColorCount {
                    color: colorspace.to_float(*color),
                    count: *count,
                }
            })
            .collect()
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&Color, &usize)> + 'a> {
        Box::new(self.data.iter())
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
    use super::super::*;

    #[test]
    fn count_duplicates() {
        let mut hist: Histogram =
            [Color::new(10, 20, 30, 99), Color::new(0, 99, 0, 99)].iter().cloned().collect();
        hist.extend([Color::new(20, 0, 40, 99), Color::new(0, 99, 0, 99)].iter().cloned());
        assert_eq!(*hist.data.get(&Color::new(0, 99, 0, 99)).unwrap(), 2usize);
    }
}
