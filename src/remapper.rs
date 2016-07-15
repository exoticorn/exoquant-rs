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
