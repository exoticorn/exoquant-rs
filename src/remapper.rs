use super::*;

pub struct Remapper<'a, T: 'a + ColorSpace, D: 'a + Ditherer> {
    map: ColorMap,
    colorspace: &'a T,
    ditherer: &'a D,
}

impl<'a, T: ColorSpace, D: Ditherer> Remapper<'a, T, D> {
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

pub struct RemapIter<'a, I, D, C>
    where I: Iterator<Item = Color>,
          D: ditherer::LinearDitherer + 'a,
          C: ColorSpace + 'a
{
    iter: I,
    ditherer: &'a D,
    state: D::State,
    colormap: &'a ColorMap,
    colorspace: &'a C,
    i: usize,
    width: usize,
}

impl<'a, I, D, C> Iterator for RemapIter<'a, I, D, C>
    where I: Iterator<Item = Color>,
          D: ditherer::LinearDitherer,
          C: ColorSpace
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(color) = self.iter.next() {
            let x = self.i % self.width;
            let y = self.i / self.width;
            self.i += 1;
            let color = self.colorspace.to_float(color);
            Some(self.ditherer
                .remap_linear(&mut self.state, self.colormap, self.colorspace, color, x, y))
        } else {
            None
        }
    }
}

pub trait RemapperIter<'a, D, C>
    where D: ditherer::LinearDitherer + 'a,
          C: ColorSpace + 'a
{
    fn remap_iter<I>(&'a self, width: usize, iter: I) -> RemapIter<'a, I, D, C>
        where I: Iterator<Item = Color>;
}

impl<'a, D, C> RemapperIter<'a, D, C> for Remapper<'a, C, D>
    where D: ditherer::LinearDitherer + 'a,
          C: ColorSpace + 'a
{
    fn remap_iter<I>(&'a self, width: usize, iter: I) -> RemapIter<'a, I, D, C>
        where I: Iterator<Item = Color>
    {
        RemapIter {
            iter: iter,
            ditherer: self.ditherer,
            state: D::new_state(width),
            colormap: &self.map,
            colorspace: self.colorspace,
            i: 0,
            width: width,
        }
    }
}
