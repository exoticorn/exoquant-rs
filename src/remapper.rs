use super::*;

pub struct Remapper<'a, T: 'a + ColorSpace, D: 'a + Ditherer + ?Sized> {
    map: ColorMap,
    colorspace: &'a T,
    ditherer: &'a D,
}

impl<'a, T: ColorSpace, D: Ditherer + ?Sized> Remapper<'a, T, D> {
    pub fn new(palette: &[Color], colorspace: &'a T, ditherer: &'a D) -> Remapper<'a, T, D> {
        Remapper {
            map: ColorMap::new(palette, colorspace),
            colorspace: colorspace,
            ditherer: ditherer,
        }
    }

    pub fn remap_usize(&self, image: &[Color], width: usize) -> Vec<usize> {
        self.ditherer.remap(&self.map, self.colorspace, image, width)
    }

    pub fn remap(&self, image: &[Color], width: usize) -> Vec<u8> {
        assert!(self.map.num_colors() <= 256);
        self.remap_usize(image, width).iter().map(|i| *i as u8).collect()
    }
}
