use super::*;
use ditherer::Ditherer;

/// A helper type to very slightly simplify remapping images using a `Ditherer`.
///
/// The plain `remap` function remaps a `&[Color]` to a `Vec<u8>`, while
/// the `remap_iter` function remaps a `Box<Iterator<Item = Color>>` to a
/// `Box<Iterator<Item = u8>>`. The `_usize` functions remap to `usize` instead of `u8`,
/// in case you need palettes with more than 256 colors.
///
/// # Examples
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// # let histogram: Histogram = image.pixels.iter().cloned().collect();
/// # let colorspace = SimpleColorSpace::default();
/// # let palette = generate_palette(&histogram, &colorspace, &optimizer::None, 256);
/// let ditherer = ditherer::FloydSteinberg::new();
/// let remapper = Remapper::new(&palette, &colorspace, &ditherer);
/// let indexed_image_data = remapper.remap(&image.pixels, image.width);
/// ```
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// # let histogram: Histogram = image.pixels.iter().cloned().collect();
/// # let colorspace = SimpleColorSpace::default();
/// # let palette = generate_palette(&histogram, &colorspace, &optimizer::None, 256);
/// let ditherer = ditherer::FloydSteinberg::new();
/// let remapper = Remapper::new(&palette, &colorspace, &ditherer);
/// let iter = remapper.remap_iter(Box::new(image.pixels.iter().cloned()), image.width);
/// let indexed_image_data: Vec<u8> = iter.collect();
/// ```
pub struct Remapper<'a, T: 'a + ColorSpace, D: 'a + Ditherer + ?Sized> {
    map: ColorMap,
    colorspace: &'a T,
    ditherer: &'a D,
}

impl<'a, T: ColorSpace, D: Ditherer + ?Sized> Remapper<'a, T, D> {
    /// Create a new Remapper instance for the given palette, colorspace and ditherer.
    pub fn new(palette: &[Color], colorspace: &'a T, ditherer: &'a D) -> Remapper<'a, T, D> {
        Remapper {
            map: ColorMap::new(palette, colorspace),
            colorspace: colorspace,
            ditherer: ditherer,
        }
    }

    /// Remap and dither a `&[Color]` to a `Vec<u8>`.
    pub fn remap(&self, image: &[Color], width: usize) -> Vec<u8> {
        assert!(self.map.num_colors() <= 256);
        self.ditherer
            .remap(Box::new(image.iter().map(|&c| self.colorspace.to_float(c))),
                   width,
                   &self.map,
                   self.colorspace)
            .map(|i| i as u8)
            .collect()
    }

    /// Remap and dither a `&[Color]` to a `Vec<usize>`.
    pub fn remap_usize(&self, image: &[Color], width: usize) -> Vec<usize> {
        self.ditherer
            .remap(Box::new(image.iter().map(|&c| self.colorspace.to_float(c))),
                   width,
                   &self.map,
                   self.colorspace)
            .collect()
    }

    /// Remap and dither a `Box<Iterator<Item = Color>>` to a `Box<Iterator<Item = u8>>`.
    pub fn remap_iter<'b>(&'b self,
                          image: Box<dyn Iterator<Item = Color> + 'b>,
                          width: usize)
                          -> Box<dyn Iterator<Item = u8> + 'b> {
        assert!(self.map.num_colors() <= 256);
        Box::new(self.ditherer
            .remap(Box::new(image.map(move |c| self.colorspace.to_float(c))),
                   width,
                   &self.map,
                   self.colorspace)
            .map(|i| i as u8))
    }

    /// Remap and dither a `Box<Iterator<Item = Color>>` to a `Box<Iterator<Item = usize>>`.
    pub fn remap_iter_usize<'b>(&'b self,
                                image: Box<dyn Iterator<Item = Color> + 'b>,
                                width: usize)
                                -> Box<dyn Iterator<Item = usize> + 'b> {
        assert!(self.map.num_colors() <= 256);
        self.ditherer
            .remap(Box::new(image.map(move |c| self.colorspace.to_float(c))),
                   width,
                   &self.map,
                   self.colorspace)

    }
}
