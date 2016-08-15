use super::*;

pub trait Ditherer {
    fn remap(&self,
             map: &ColorMap,
             colorspace: &ColorSpace,
             image: &[Color],
             width: usize)
             -> Vec<usize>;
}

pub struct None;
impl Ditherer for None {
    fn remap(&self,
             map: &ColorMap,
             colorspace: &ColorSpace,
             image: &[Color],
             _: usize)
             -> Vec<usize> {
        image.iter().map(|&c| map.find_nearest(colorspace.to_float(c))).collect()
    }
}

pub struct Ordered;
const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];
impl Ditherer for Ordered {
    fn remap(&self,
             map: &ColorMap,
             colorspace: &ColorSpace,
             image: &[Color],
             width: usize)
             -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, &c)| {
                let x = i % width;
                let y = i / width;
                let color = colorspace.to_float(c);
                let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
                let i = map.find_nearest(color);
                let d = map.neighbor_distance(i);
                let color = color + (d * dither * 0.75);
                map.find_nearest(color)
            })
            .collect()
    }
}

pub struct FloydSteinberg(f64, f64, f64, f64, f64);
impl FloydSteinberg {
    pub fn new() -> FloydSteinberg {
        FloydSteinberg(7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0, 0.8)
    }
    pub fn checkered() -> FloydSteinberg {
        FloydSteinberg(7.0 / 16.0, 1.5 / 16.0, 6.5 / 16.0, 1.0 / 16.0, 0.5)
    }
}
impl Ditherer for FloydSteinberg {
    fn remap(&self,
             map: &ColorMap,
             colorspace: &ColorSpace,
             image: &[Color],
             width: usize)
             -> Vec<usize> {
        let mut errors: Vec<Colorf> = (0..width * 2).map(|_| Colorf::zero()).collect();
        image.iter()
            .enumerate()
            .map(|(i, &c)| {
                let x = i % width;
                let y = i / width;
                let c = colorspace.to_float(c);
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
            })
            .collect()
    }
}
