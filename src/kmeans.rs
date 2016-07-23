use ::color::FloatColor;
use ::color::Color;
use ::quantizer::HistColor;
use ::colormap::ColorMap;
use ::histogram::Histogram;
use ::colorspace::ColorSpace;

struct KMeansCluster {
    sum: FloatColor,
    count: usize,
}

pub fn kmeans_step(colors: Vec<FloatColor>, histogram: &[HistColor]) -> Vec<FloatColor> {
    let map = ColorMap::from_float_colors(colors.iter().cloned().collect());
    let mut clusters: Vec<_> = (0..colors.len())
        .map(|_| {
            KMeansCluster {
                sum: FloatColor::default(),
                count: 0,
            }
        })
        .collect();
    for entry in histogram {
        let index = map.find_nearest(entry.color);
        let mut cluster = &mut clusters[index];
        cluster.sum += entry.color * entry.count as f64;
        cluster.count += entry.count;
    }
    clusters.iter().map(|cluster| cluster.sum * (1.0 / cluster.count as f64)).collect()
}

pub fn optimize_palette<T: ColorSpace>(colorspace: &T,
                                       palette: Vec<Color>,
                                       histogram: &Histogram,
                                       num_iterations: usize)
                                       -> Vec<Color> {
    let hist = histogram.to_hist_colors(colorspace);
    let mut colors = palette.iter().map(|c| colorspace.to_float(*c)).collect();
    for _ in 0..num_iterations {
        colors = kmeans_step(colors, &hist);
    }
    colors.iter().map(|&c| colorspace.from_float(c)).collect()
}
