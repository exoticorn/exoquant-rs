use ::color::Color;
use ::color::FloatColor;
use ::colormap::ColorMap;

pub trait Remapper {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize>;
    fn remap8(&self, image: &[Color], width: usize) -> Vec<u8> {
        self.remap(image, width).iter().map(|i| *i as u8).collect()
    }
}

pub struct RemapperNoDither {
    map: ColorMap,
}

impl RemapperNoDither {
    pub fn new(palette: &[Color]) -> RemapperNoDither {
        RemapperNoDither { map: ColorMap::new(palette) }
    }
}

impl Remapper for RemapperNoDither {
    fn remap(&self, image: &[Color], _: usize) -> Vec<usize> {
        image.iter().map(|c| self.map.find_nearest(c)).collect()
    }
}

pub struct RemapperOrdered {
    map: ColorMap,
}

impl RemapperOrdered {
    pub fn new(palette: &[Color]) -> RemapperOrdered {
        RemapperOrdered { map: ColorMap::new(palette) }
    }
}

const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];

impl Remapper for RemapperOrdered {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = i / width;
                let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
                let color: FloatColor = c.into();
                let i = self.map.find_nearest_float(color);
                let d = self.map.neighbor_distance(i);
                let color = color + (d * dither * 1.0);
                self.map.find_nearest_float(color)
            })
            .collect()
    }
}

pub struct RemapperOrdered2 {
    map: ColorMap,
}

impl RemapperOrdered2 {
    pub fn new(palette: &[Color]) -> RemapperOrdered2 {
        RemapperOrdered2 { map: ColorMap::new(palette) }
    }
}

const DITHER_MATRIX2: [u32; 4] = [0, 2, 3, 1];

impl Remapper for RemapperOrdered2 {
    fn remap(&self, image: &[Color], width: usize) -> Vec<usize> {
        image.iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % width;
                let y = i / width;
                let color: FloatColor = c.into();
                let i = self.map.find_nearest_float(color);
                let c = self.map.float_color(i);
                let diff = c - color;
                let d = diff.abs();
                if d < 0.00001 {
                    return i;
                }
                let dir = diff * (1.0 / d);
                let j = self.map.neighbor_in_dir(i, dir);
                let c2 = self.map.float_color(j);
                let d2 = (c2 - color).abs();
                let f = d / (d + d2);
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
