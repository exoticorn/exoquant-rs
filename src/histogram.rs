use std::collections::HashMap;
use std::iter::{FromIterator, IntoIterator};

use super::*;

/// A histogram that counts the number of times each color occurs in the input image data.
///
/// The Histogram is used to describe the color distribution of the input image to the
/// quantization process. Histogram implements both `Extend<Color>` and `FromIterator<Color>`,
/// so you can either create an empty instance using `Histogram::new()` and fill it width
/// `histogram.extend(iter)`, or just create from an `Iterator<Color>` using `iter.collect()`.
///
/// The `Histogram::new()`, `histogram.extend(...)` method is useful when you want to create
/// one palette for multiple distinct images, multiple frames of a GIF animation, etc.
///
/// # Examples
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// let mut histogram = Histogram::new();
/// histogram.extend(image.pixels.iter().cloned());
/// ```
///
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// let histogram: Histogram = image.pixels.iter().cloned().collect();
/// ```
pub struct Histogram {
    data: HashMap<Color, usize>,
}

/// A single float color in quantization color space with the number of times it occurs in the
/// input image data.
///
/// This type is used to hold histogram data during the actual quantization process. It's mostly
/// used internally.
#[derive(Clone)]
pub struct ColorCount {
    pub color: LabColor,
    pub count: usize,
}

impl Histogram {
    /// Returns a new, empty `Histogram`.
    pub fn new() -> Histogram {
        Histogram {
            data: HashMap::new(),
        }
    }

    /// Converts the rgba8 `Histogram` to a Vec of `ColorCount` in quantization color space.
    ///
    /// Mostly used internally.
    pub fn to_color_counts(&self) -> Vec<ColorCount> {
        self.data
            .iter()
            .map(|(color, count)| ColorCount {
                color: XyzColor::from(SrgbColor::from(*color)).into(),
                count: *count,
            }).collect()
    }

    /// Returns an iterator over the histogram data.
    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (&Color, &usize)> + 'a> {
        Box::new(self.data.iter())
    }
}

impl Extend<Color> for Histogram {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Color>,
    {
        for pixel in iter {
            let count = self.data.entry(pixel).or_insert(0);
            *count += 1;
        }
    }
}

impl FromIterator<Color> for Histogram {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Color>,
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
        let mut hist: Histogram = [Color::new(10, 20, 30, 99), Color::new(0, 99, 0, 99)]
            .iter()
            .cloned()
            .collect();
        hist.extend(
            [Color::new(20, 0, 40, 99), Color::new(0, 99, 0, 99)]
                .iter()
                .cloned(),
        );
        assert_eq!(*hist.data.get(&Color::new(0, 99, 0, 99)).unwrap(), 2usize);
    }
}
