use super::*;
use std::mem;

pub trait Ditherer {
    fn remap<T: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &T,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize>;
}

pub trait LinearDitherer {
    type State;
    fn new_state(width: usize) -> Self::State;
    fn remap_linear<T: ColorSpace>(&self,
                                   state: &mut Self::State,
                                   map: &ColorMap,
                                   colorspace: &T,
                                   color: Colorf,
                                   x: usize,
                                   y: usize)
                                   -> usize;
}

impl<T> Ditherer for T
    where T: LinearDitherer
{
    fn remap<C: ColorSpace>(&self,
                            map: &ColorMap,
                            colorspace: &C,
                            image: &[Color],
                            width: usize)
                            -> Vec<usize> {
        let mut state = T::new_state(width);
        image.iter()
            .enumerate()
            .map(|(i, &c)| {
                let x = i % width;
                let y = i / width;
                self.remap_linear(&mut state, map, colorspace, colorspace.to_float(c), x, y)
            })
            .collect()
    }
}

pub trait StatelessDitherer {
    fn remap_stateless<T: ColorSpace>(&self,
                                      map: &ColorMap,
                                      colorspace: &T,
                                      color: Colorf,
                                      x: usize,
                                      y: usize)
                                      -> usize;
}

impl<T> LinearDitherer for T
    where T: StatelessDitherer
{
    type State = ();
    fn new_state(_: usize) -> () {
        ()
    }
    fn remap_linear<C: ColorSpace>(&self,
                                   _: &mut (),
                                   map: &ColorMap,
                                   colorspace: &C,
                                   color: Colorf,
                                   x: usize,
                                   y: usize)
                                   -> usize {
        self.remap_stateless(map, colorspace, color, x, y)
    }
}

pub struct None;
impl StatelessDitherer for None {
    fn remap_stateless<T: ColorSpace>(&self,
                                      map: &ColorMap,
                                      _: &T,
                                      color: Colorf,
                                      _: usize,
                                      _: usize)
                                      -> usize {
        map.find_nearest(color)
    }
}

pub struct FastOrdered;
const DITHER_MATRIX: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];
impl StatelessDitherer for FastOrdered {
    fn remap_stateless<T: ColorSpace>(&self,
                                      map: &ColorMap,
                                      _: &T,
                                      color: Colorf,
                                      x: usize,
                                      y: usize)
                                      -> usize {
        let dither = DITHER_MATRIX[(x & 1) + (y & 1) * 2];
        let i = map.find_nearest(color);
        let d = map.neighbor_distance(i);
        let color = color + (d * dither * 0.75);
        map.find_nearest(color)
    }
}

pub struct Ordered;
const DITHER_MATRIX2: [usize; 4] = [0, 2, 3, 1];
impl StatelessDitherer for Ordered {
    fn remap_stateless<T: ColorSpace>(&self,
                                      map: &ColorMap,
                                      colorspace: &T,
                                      color: Colorf,
                                      x: usize,
                                      y: usize)
                                      -> usize {
        let mut indices = [0usize, 0, 0, 0];
        let mut error = Colorf::zero();
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
        let dither = DITHER_MATRIX2[(x & 1) + (y & 1) * 2];
        indices[dither]
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
pub struct FloydSteinbergState {
    errors: Vec<Colorf>,
    width: usize,
}
impl LinearDitherer for FloydSteinberg {
    type State = FloydSteinbergState;
    fn new_state(width: usize) -> Self::State {
        FloydSteinbergState {
            errors: (0..width * 2).map(|_| Colorf::zero()).collect(),
            width: width,
        }
    }
    fn remap_linear<T: ColorSpace>(&self,
                                   state: &mut Self::State,
                                   map: &ColorMap,
                                   colorspace: &T,
                                   c: Colorf,
                                   x: usize,
                                   y: usize)
                                   -> usize {
        let y = y & 1;
        let errors = &mut state.errors;
        let width = state.width;
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
    }
}
