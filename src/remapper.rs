use ::color::Color;
use ::color::FloatColor;
use ::colormap::ColorMap;
use ::colorspace::ColorSpace;
use std::mem;

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

    pub fn remap(&self, image: &[Color], width: usize) -> Vec<usize> {
        self.ditherer.remap(&self.map, self.colorspace, image, width)
    }

    pub fn remap8(&self, image: &[Color], width: usize) -> Vec<u8> {
        self.remap(image, width).iter().map(|i| *i as u8).collect()
    }
}

pub trait Ditherer {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize>;
}

pub struct DithererNone;
impl Ditherer for DithererNone {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            _: usize)
                            -> Vec<usize> {
        image.iter().map(|c| map.find_nearest(colorspace.to_float(*c))).collect()
    }
}

pub struct DithererOrdered;
const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];
impl Ditherer for DithererOrdered {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = i / width;
                let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
                let color: FloatColor = colorspace.to_float(*c);
                let i = map.find_nearest(color);
                let d = map.neighbor_distance(i);
                let color = color + (d * dither * 0.75);
                map.find_nearest(color)
            })
            .collect()
    }
}

pub struct DithererOrdered2;
const DITHER_MATRIX2: [usize; 4] = [0, 2, 3, 1];
impl Ditherer for DithererOrdered2 {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize> {
        let mut indices = [0usize, 0, 0, 0];
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let color: FloatColor = colorspace.to_float(*c);
                let mut error = FloatColor::default();
                for j in 0..4 {
                    let c = colorspace.to_dither(color) + error;
                    let mut index = map.find_nearest(colorspace.from_dither(c));
                    error = c - colorspace.to_dither(map.float_color(index));
                    for k in 0..j {
                        if indices[k] > index {
                            mem::swap(&mut index, &mut indices[k]);
                        }
                    }
                    indices[j] = index;
                }
                let x = i % width;
                let y = i / width;
                let dither = DITHER_MATRIX2[(x & 1) + (y & 1) * 2];
                indices[dither]
            })
            .collect()
    }
}

pub struct DithererFloydSteinberg(f64, f64, f64, f64, f64);
impl DithererFloydSteinberg {
    pub fn new() -> DithererFloydSteinberg {
        DithererFloydSteinberg(7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0, 0.8)
    }
    pub fn checkered() -> DithererFloydSteinberg {
        DithererFloydSteinberg(7.0 / 16.0, 1.5 / 16.0, 6.5 / 16.0, 1.0 / 16.0, 0.5)
    }
}
impl Ditherer for DithererFloydSteinberg {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize> {
        let mut errors: Vec<_> = (0..(width * 2)).map(|_| FloatColor::default()).collect();
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = (i / width) & 1;
                let row = y * width;
                let other = (y ^ 1) * width;
                let c = colorspace.to_float(*c);
                let c = colorspace.to_dither(c);
                let index = map.find_nearest(colorspace.from_dither(c + errors[row + x]));
                let c2 = map.float_color(index);
                let error = c + errors[row + x] * self.4 - colorspace.to_dither(c2);
                errors[row + (x + 1) % width] += error * self.0;
                errors[other + (x + 1) % width] = error * self.3;
                errors[other + x] += error * self.2;
                errors[other + (x + width - 1) % width] += error * self.1;
                index
            })
            .collect()
    }
}
