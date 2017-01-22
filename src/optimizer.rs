//! K-Means optimization

use super::*;
use std::f64;

/// An interface for K-Means optimizers.
pub trait Optimizer {
    /// Do one K-Means optimization step and return colors that better represent the histogram.
    ///
    /// This is the one function custom implementations have to provide.
    fn step(&self, colors: Vec<Colorf>, histogram: &[ColorCount]) -> Vec<Colorf>;

    /// Optimize a given palette with a number of K-Means iteration.
    ///
    /// # Examples:
    /// ```
    /// # use exoquant::*;
    /// # use exoquant::optimizer::Optimizer;
    /// # let image = testdata::test_image();
    /// # let histogram: Histogram = image.pixels.iter().cloned().collect();
    /// # let colorspace = SimpleColorSpace::default();
    /// let palette = Quantizer::create_palette(&histogram, &colorspace, 256);
    /// let palette = optimizer::KMeans.optimize_palette(&colorspace, &palette,
    ///   &histogram, 16);
    /// ```
    fn optimize_palette(&self,
                        colorspace: &ColorSpace,
                        palette: &[Color],
                        histogram: &Histogram,
                        num_iterations: usize)
                        -> Vec<Color> {
        if self.is_noop() {
            return palette.iter().cloned().collect();
        }
        let hist = histogram.to_color_counts(colorspace);
        let mut colors =
            palette.iter().map(|c| colorspace.output_to_quantization((*c).into())).collect();
        for _ in 0..num_iterations {
            colors = self.step(colors, &hist);
        }
        colors.iter().map(|&c| colorspace.quantization_to_output(c).into()).collect()
    }

    /// Returns whether this Optimizer is a No-op implementation.
    ///
    /// This is used to shortcut some functions that take an Optimizer as a paramter if
    /// the Optimizer won't do anything anyway.
    fn is_noop(&self) -> bool {
        false
    }
}

/// A No-op Optimizer implementation.
pub struct None;

impl Optimizer for None {
    fn step(&self, colors: Vec<Colorf>, _: &[ColorCount]) -> Vec<Colorf> {
        colors
    }

    fn is_noop(&self) -> bool {
        true
    }
}

struct KMeansCluster {
    sum: Colorf,
    weight: f64,
}

/// A standard K-Means Optimizer.
///
/// Each palette entry is moved toward the average of the cluster of input colors it is
/// going to represent to find a locally optimal palette.
pub struct KMeans;

impl Optimizer for KMeans {
    fn step(&self, colors: Vec<Colorf>, histogram: &[ColorCount]) -> Vec<Colorf> {
        let map = ColorMap::from_float_colors(colors.iter().cloned().collect());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| {
                KMeansCluster {
                    sum: Colorf::zero(),
                    weight: 0.0,
                }
            })
            .collect();
        for entry in histogram {
            let index = map.find_nearest(entry.color);
            let mut cluster = &mut clusters[index];
            cluster.sum += entry.color * entry.count as f64;
            cluster.weight += entry.count as f64;
        }
        clusters.iter()
            .map(|cluster| cluster.sum * (1.0 / cluster.weight.max(1.0)))
            .collect()
    }
}

/// A slightly experimental Optimizer that improves color representation in dithered images.
///
/// The standard K-Means optimization produces palettes that can't represent the extrema of the
/// input colors, even with dithering. This optimizer tries to optimize the representation of
/// these fringe colors. This does increase the dithering noise a bit and is only really
/// useful for low target color counts (say <= 64).
pub struct WeightedKMeans;

impl Optimizer for WeightedKMeans {
    fn step(&self, mut colors: Vec<Colorf>, histogram: &[ColorCount]) -> Vec<Colorf> {
        let map = ColorMap::from_float_colors(colors.clone());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| {
                KMeansCluster {
                    sum: Colorf::zero(),
                    weight: 0.0,
                }
            })
            .collect();
        for entry in histogram {
            let index = map.find_nearest(entry.color);
            let neighbors = map.neighbors(index);
            let mut error_sum = Colorf::zero();
            let mut color = entry.color;
            for _ in 0..4 {
                let mut best_i = 0;
                let mut best_error = f64::MAX;
                for &i in neighbors {
                    let diff = color - colors[i];
                    let error = diff.abs();
                    if error < best_error {
                        best_i = i;
                        best_error = error;
                    }
                }
                let diff = color - colors[best_i];
                error_sum += diff;
                color = entry.color + diff;
            }
            let mut cluster = &mut clusters[index];
            let weight = entry.count as f64 * error_sum.dot(&error_sum);
            cluster.sum += entry.color * weight;
            cluster.weight += weight;
        }
        for i in 0..colors.len() {
            let mut cluster = &mut clusters[i];
            if cluster.weight > 0.0 {
                colors[i] = cluster.sum * (1.0 / cluster.weight);
            }
            cluster.sum = Colorf::zero();
            cluster.weight = 0.0;
        }
        colors
    }
}
