//! K-Means optimization

use super::*;
use std::f32;

/// An interface for K-Means optimizers.
pub trait Optimizer {
    /// Do one K-Means optimization step and return colors that better represent the histogram.
    ///
    /// This is the one function custom implementations have to provide.
    fn step(&self, colors: Vec<LabColor>, histogram: &[ColorCount]) -> Vec<LabColor>;

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
    fn optimize_palette(
        &self,
        palette: &[Color],
        histogram: &Histogram,
        num_iterations: usize,
    ) -> Vec<Color> {
        if self.is_noop() {
            return palette.iter().cloned().collect();
        }
        let hist = histogram.to_color_counts();
        let mut colors: Vec<LabColor> = palette
            .iter()
            .map(|&c| XyzColor::from(SrgbColor::from(c)).into())
            .collect();
        for _ in 0..num_iterations {
            colors = self.step(colors, &hist);
        }
        colors.iter().map(|&c| c.into()).collect()
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
    fn step(&self, colors: Vec<LabColor>, _: &[ColorCount]) -> Vec<LabColor> {
        colors
    }

    fn is_noop(&self) -> bool {
        true
    }
}

struct KMeansCluster {
    sum: Vec4<LabColor>,
    weight: f32,
}

/// A standard K-Means Optimizer.
///
/// Each palette entry is moved toward the average of the cluster of input colors it is
/// going to represent to find a locally optimal palette.
pub struct KMeans;

impl Optimizer for KMeans {
    fn step(&self, colors: Vec<LabColor>, histogram: &[ColorCount]) -> Vec<LabColor> {
        let map = ColorMap::from_float_colors(colors.iter().cloned().collect());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| KMeansCluster {
                sum: Vec4::zero(),
                weight: 0.0,
            }).collect();
        for entry in histogram {
            let index = map.find_nearest(entry.color);
            let mut cluster = &mut clusters[index];
            cluster.sum += entry.color.to_vec4() * entry.count as f32;
            cluster.weight += entry.count as f32;
        }
        clusters
            .iter()
            .map(|cluster| (cluster.sum * (1.0 / cluster.weight.max(1.0))).to_color())
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
    fn step(&self, mut colors: Vec<LabColor>, histogram: &[ColorCount]) -> Vec<LabColor> {
        let map = ColorMap::from_float_colors(colors.clone());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| KMeansCluster {
                sum: Vec4::zero(),
                weight: 0.0,
            }).collect();
        for entry in histogram {
            let index = map.find_nearest(entry.color);
            let neighbors = map.neighbors(index);
            let mut error_sum = Vec4::<LabColor>::zero();
            let mut color = entry.color.to_vec4();
            for _ in 0..4 {
                let mut best_i = 0;
                let mut best_error = f32::MAX;
                for &i in neighbors {
                    let diff = color - colors[i].to_vec4();
                    let error = diff.abs();
                    if error < best_error {
                        best_i = i;
                        best_error = error;
                    }
                }
                let diff = color - colors[best_i].to_vec4();
                error_sum += diff;
                color = entry.color.to_vec4() + diff;
            }
            let mut cluster = &mut clusters[index];
            let weight = entry.count as f32 * error_sum.dot(error_sum);
            cluster.sum += entry.color.to_vec4() * weight;
            cluster.weight += weight;
        }
        for i in 0..colors.len() {
            let mut cluster = &mut clusters[i];
            if cluster.weight > 0.0 {
                colors[i] = (cluster.sum * (1.0 / cluster.weight)).to_color();
            }
            cluster.sum = Vec4::zero();
            cluster.weight = 0.0;
        }
        colors
    }
}
