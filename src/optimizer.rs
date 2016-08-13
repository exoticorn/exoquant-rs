use ::color::FloatColor;
use ::color::Color;
use ::quantizer::HistColor;
use ::colormap::ColorMap;
use ::histogram::Histogram;
use ::colorspace::ColorSpace;
use std::f64;

pub trait Optimizer {
    fn step(&self, colors: Vec<FloatColor>, histogram: &[HistColor]) -> Vec<FloatColor>;

    fn optimize_palette<C: ColorSpace>(&self,
                                       colorspace: &C,
                                       palette: &[Color],
                                       histogram: &Histogram,
                                       num_iterations: usize)
                                       -> Vec<Color> {
        if self.is_noop() {
            return palette.iter().cloned().collect();
        }
        let hist = histogram.to_hist_colors(colorspace);
        let mut colors = palette.iter().map(|c| colorspace.to_float(*c)).collect();
        for _ in 0..num_iterations {
            colors = self.step(colors, &hist);
        }
        colors.iter().map(|&c| colorspace.from_float(c)).collect()
    }

    fn is_noop(&self) -> bool {
        false
    }
}

pub struct None;

impl Optimizer for None {
    fn step(&self, colors: Vec<FloatColor>, _: &[HistColor]) -> Vec<FloatColor> {
        colors
    }

    fn is_noop(&self) -> bool {
        true
    }
}

struct KMeansCluster {
    sum: FloatColor,
    weight: f64,
}

pub struct KMeans;

impl Optimizer for KMeans {
    fn step(&self, colors: Vec<FloatColor>, histogram: &[HistColor]) -> Vec<FloatColor> {
        let map = ColorMap::from_float_colors(colors.iter().cloned().collect());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| {
                KMeansCluster {
                    sum: FloatColor::default(),
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
        clusters.iter().map(|cluster| cluster.sum * (1.0 / cluster.weight)).collect()
    }
}

pub struct WeightedKMeans;

impl Optimizer for WeightedKMeans {
    fn step(&self, mut colors: Vec<FloatColor>, histogram: &[HistColor]) -> Vec<FloatColor> {
        let map = ColorMap::from_float_colors(colors.clone());
        let mut clusters: Vec<_> = (0..colors.len())
            .map(|_| {
                KMeansCluster {
                    sum: FloatColor::default(),
                    weight: 0.0,
                }
            })
            .collect();
        for entry in histogram {
            let index = map.find_nearest(entry.color);
            let neighbors = map.neighbors(index);
            let mut error_sum = FloatColor::default();
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
            cluster.sum = FloatColor::default();
            cluster.weight = 0.0;
        }
        colors
    }
}
