use ::color::Color;
use ::color::FloatColor;
use ::colormap::ColorMap;
use ::colorspace::ColorSpace;

pub trait Remapper {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize>;
    fn remap8(&self, image: &[Color], width: usize) -> Vec<u8> {
        self.remap(image, width).iter().map(|i| *i as u8).collect()
    }
}

pub struct RemapperNoDither<'a, T: 'a + ColorSpace> {
    map: ColorMap,
    colorspace: &'a T,
}

impl<'a, T: ColorSpace> RemapperNoDither<'a, T> {
    pub fn new(palette: &[Color], colorspace: &'a T) -> RemapperNoDither<'a, T> {
        RemapperNoDither {
            map: ColorMap::new(palette, colorspace),
            colorspace: colorspace,
        }
    }
}

impl<'a, T: ColorSpace> Remapper for RemapperNoDither<'a, T> {
    fn remap(&self, image: &[Color], _: usize) -> Vec<usize> {
        image.iter().map(|c| self.map.find_nearest(self.colorspace.to_float(*c))).collect()
    }
}

pub struct RemapperOrdered<'a, T: 'a + ColorSpace> {
    map: ColorMap,
    colorspace: &'a T,
}

impl<'a, T: ColorSpace> RemapperOrdered<'a, T> {
    pub fn new(palette: &[Color], colorspace: &'a T) -> RemapperOrdered<'a, T> {
        RemapperOrdered {
            map: ColorMap::new(palette, colorspace),
            colorspace: colorspace,
        }
    }
}

const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];

impl<'a, T: ColorSpace> Remapper for RemapperOrdered<'a, T> {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = i / width;
                let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
                let color: FloatColor = self.colorspace.to_float(*c);
                let i = self.map.find_nearest(color);
                let d = self.map.neighbor_distance(i);
                let color = color + (d * dither * 0.75);
                self.map.find_nearest(color)
            })
            .collect()
    }
}

pub struct RemapperOrdered2<'a, T: 'a + ColorSpace> {
    map: ColorMap,
    colorspace: &'a T,
}

impl<'a, T: ColorSpace> RemapperOrdered2<'a, T> {
    pub fn new(palette: &[Color], colorspace: &'a T) -> RemapperOrdered2<'a, T> {
        RemapperOrdered2 {
            map: ColorMap::new(palette, colorspace),
            colorspace: colorspace,
        }
    }
}

const DITHER_MATRIX2: [u32; 4] = [0, 2, 3, 1];

impl<'a, T: ColorSpace> Remapper for RemapperOrdered2<'a, T> {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = i / width;
                let color: FloatColor = self.colorspace.to_float(*c);
                let i = self.map.find_nearest(color);
                let c = self.map.float_color(i);
                let diff = color - c;
                let d = diff.abs();
                if d < 0.00001 {
                    return i;
                }
                let dir = diff * (1.0 / d);
                let j = self.map.neighbor_in_dir(i, dir);
                let c2 = self.map.float_color(j);
                let span = c2 - c;
                let f = (color - c).dot(&span) / span.dot(&span);
                let offset = if f > 0.375 {
                    2
                } else if f > 0.125 {
                    1
                } else {
                    0
                };
                let mut dither = DITHER_MATRIX2[(x & 1) + (y & 1) * 2];
                if j < i {
                    dither = 3 - dither;
                }
                if offset > dither { j } else { i }
            })
            .collect()
    }
}
