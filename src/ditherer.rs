//! Dithered remapping

use super::*;

/// An interface for dithered color remapping.
///
/// End users of this library will most likely not call `ditherer::remap` directly, but rather use
/// the `Remapper` helper.
pub trait Ditherer {
    /// Remaps an iterator of input pixel (float-)colors of an Image to an Iterator of palette
    /// indices.
    fn remap<'a>(&'a self,
                 image: Box<Iterator<Item = Colorf> + 'a>,
                 width: usize,
                 map: &'a ColorMap,
                 colorspace: &'a ColorSpace)
                 -> Box<Iterator<Item = usize> + 'a>;
}

/// A Ditherer that simply remaps each pixel to the nearest palette index without any actual
/// dithering.
pub struct None;
impl Ditherer for None {
    fn remap<'a>(&'a self,
                 image: Box<Iterator<Item = Colorf> + 'a>,
                 _: usize,
                 map: &'a ColorMap,
                 _: &'a ColorSpace)
                 -> Box<Iterator<Item = usize> + 'a> {
        Box::new(image.map(move |c| map.find_nearest(c)))
    }
}

/// A 2x2 ordered dithering.
///
/// An ordered ditherer features slightly worse dithering quality than a floyd-steinberg ditherer,
/// but might look more pleasing as it appears less like random noise. An ordered dithered image
/// also has the advantage of compressing a lot better than floyd-steinberg dithered ones.
///
/// Note: don't use ordered dithering on images that are intended to be down-scaled later or risk
/// moire artifacts.
pub struct Ordered;
const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];
impl Ditherer for Ordered {
    fn remap<'a>(&'a self,
                 image: Box<Iterator<Item = Colorf> + 'a>,
                 width: usize,
                 map: &'a ColorMap,
                 _: &'a ColorSpace)
                 -> Box<Iterator<Item = usize> + 'a> {
        Box::new(image.enumerate()
            .map(move |(i, color)| {
                let x = i % width;
                let y = i / width;
                let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
                let i = map.find_nearest(color);
                let d = map.neighbor_distance(i);
                let color = color + (d * dither * 0.75);
                map.find_nearest(color)
            }))
    }
}

/// A few variants of a Floyd-Steinberg ditherer.
pub struct FloydSteinberg(f64, f64, f64, f64, f64);
impl FloydSteinberg {
    /// Returns a floyd-steinberg variant that reduces color bleeding.
    pub fn new() -> FloydSteinberg {
        FloydSteinberg(7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0, 0.8)
    }
    /// Returns a vanilla floyd-steinberg ditherer as originally described.
    pub fn vanilla() -> FloydSteinberg {
        FloydSteinberg(7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0, 1.0)
    }
    /// Returns a modified floyd-steinber ditherer slightly favoring checker board patterns.
    ///
    /// The resulting dithering looks a little less like random noise. Don't use for images
    /// that are later down-scaled, as that will risk moire artifacts.
    pub fn checkered() -> FloydSteinberg {
        FloydSteinberg(7.0 / 16.0, 1.5 / 16.0, 6.5 / 16.0, 1.0 / 16.0, 0.5)
    }
}
impl Ditherer for FloydSteinberg {
    fn remap<'a>(&'a self,
                 image: Box<Iterator<Item = Colorf> + 'a>,
                 width: usize,
                 map: &'a ColorMap,
                 colorspace: &'a ColorSpace)
                 -> Box<Iterator<Item = usize> + 'a> {
        let mut errors: Vec<Colorf> = (0..width * 2).map(|_| Colorf::zero()).collect();
        Box::new(image.enumerate()
            .map(move |(i, c)| {
                let x = i % width;
                let y = i / width;
                let y = y & 1;
                let row = y * width;
                let other = (y ^ 1) * width;
                let c = colorspace.to_dither(c);
                let index = map.find_nearest(colorspace.from_dither(c + errors[row + x]));
                let c2 = map.float_color(index);
                let error = c + errors[row + x] * self.4 - colorspace.to_dither(c2);
                errors[row + (x + 1) % width] += error * self.0;
                errors[other + (x + 1) % width] = error * self.3;
                errors[other + x] += error * self.2;
                errors[other + (x + width - 1) % width] += error * self.1;
                index
            }))
    }
}
