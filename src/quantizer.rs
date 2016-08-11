use ::color::FloatColor;
use ::color::Color;
use ::colorspace::ColorSpace;
use ::kmeans::KMeans;
use ::colormap::ColorMap;

#[derive(Clone)]
pub struct HistColor {
    pub color: FloatColor,
    pub count: usize,
}

struct QuantizerNode {
    histogram: Vec<HistColor>, // a histogram of the colors represented by this node
    avg: FloatColor, // the average color of this node
    vdif: f64, // the improvement to the total variance when splitting this node
    split: usize, // the best index to split this node at
}

impl QuantizerNode {
    fn new(mut histogram: Vec<HistColor>) -> QuantizerNode {
        // First calculate the color average and variance over the histogram
        let mut n = 0usize;
        let mut fsum = FloatColor::default();
        let mut fsum2 = FloatColor::default();

        for entry in &histogram {
            n += entry.count;
            fsum += entry.color * entry.count as f64;
            fsum2 += entry.color * entry.color * entry.count as f64;
        }

        if n == 0 {
            return QuantizerNode {
                histogram: histogram,
                avg: FloatColor::default(),
                vdif: 0.0,
                split: 0,
            };
        }

        let avg = fsum * (1.0 / n as f64);
        let vc = fsum2 - fsum * avg;
        let v = vc.r + vc.g + vc.b + vc.a;

        // Next sort histogram by the channel with the largest variance
        if vc.r > vc.g && vc.r > vc.b && vc.r > vc.a {
            histogram.sort_by(|a, b| a.color.r.partial_cmp(&b.color.r).unwrap());
        } else if vc.g > vc.b && vc.g > vc.a {
            histogram.sort_by(|a, b| a.color.g.partial_cmp(&b.color.g).unwrap());
        } else if vc.b > vc.a {
            histogram.sort_by(|a, b| a.color.b.partial_cmp(&b.color.b).unwrap());
        } else {
            histogram.sort_by(|a, b| a.color.a.partial_cmp(&b.color.a).unwrap());
        }

        // Determine primary vector of distribution in the histogram
        let mut dir = FloatColor::default();
        for entry in &histogram {
            let mut tmp = (entry.color - avg) * entry.count as f64;
            if tmp.dot(&dir) < 0.0 {
                tmp *= -1.0;
            }
            dir += tmp;
        }

        dir *= {
            let s = dir.dot(&dir).sqrt();
            if s < 0.000000001 {
                1.0
            } else {
                1.0 / s
            }
        };

        // Now sort histogram by primary vector
        histogram.sort_by(|a, b| a.color.dot(&dir).partial_cmp(&b.color.dot(&dir)).unwrap());

        // Find split index that results in lowest total variance
        let mut sum = FloatColor::default();
        let mut sum2 = FloatColor::default();
        let mut vdif = -v;
        let mut n2 = 0;
        let mut split = 0usize;
        for (i, entry) in histogram.iter().enumerate() {
            n2 += entry.count;
            sum += entry.color * entry.count as f64;
            sum2 += entry.color * entry.color * entry.count as f64;

            if n2 < n {
                let tmp = sum2 - sum * sum * (1.0 / n2 as f64);
                let dif_sum = fsum - sum;
                let tmp2 = (fsum2 - sum2) - dif_sum * dif_sum * (1.0 / (n - n2) as f64);
                let nv = tmp.r + tmp.g + tmp.b + tmp.a + tmp2.r + tmp2.g + tmp2.b + tmp2.a;
                if -nv > vdif {
                    vdif = -nv;
                    split = i + 1;
                }
            }
        }

        QuantizerNode {
            histogram: histogram,
            avg: avg,
            vdif: vdif + v,
            split: split,
        }
    }
}

pub struct Quantizer(Vec<QuantizerNode>);

impl Quantizer {
    pub fn new<T: ColorSpace>(histogram: &::histogram::Histogram, colorspace: &T) -> Quantizer {
        let hist = histogram.to_hist_colors(colorspace);
        Quantizer(vec![QuantizerNode::new(hist)])
    }

    pub fn create_palette<T: ColorSpace>(histogram: &::histogram::Histogram,
                                         colorspace: &T,
                                         num_colors: usize)
                                         -> Vec<Color> {
        let mut quantizer = Self::new(histogram, colorspace);
        while quantizer.num_colors() < num_colors {
            quantizer.step();
        }
        quantizer.colors(colorspace)
    }

    pub fn num_colors(&self) -> usize {
        self.0.len()
    }

    pub fn step(&mut self) {
        let (new_node1, new_node2) = {
            let node = {
                let mut best_i = 0;
                let mut best_e = 0.0;
                for i in 0..self.0.len() {
                    if self.0[i].vdif >= best_e {
                        best_e = self.0[i].vdif;
                        best_i = i;
                    }
                }
                self.0.swap_remove(best_i)
            };
            let mut colors1 = node.histogram;
            let colors2 = colors1.split_off(node.split);
            (QuantizerNode::new(colors1), QuantizerNode::new(colors2))
        };
        self.0.push(new_node1);
        self.0.push(new_node2);
    }

    pub fn colors<T: ColorSpace>(&self, colorspace: &T) -> Vec<Color> {
        self.0.iter().map(|node| colorspace.from_float(node.avg)).collect()
    }

    pub fn do_kmeans_optimization<K>(self, kmeans: &K, num_iterations: usize) -> Quantizer
        where K: KMeans
    {
        if kmeans.is_noop() {
            return self;
        }
        let (mut colors, histograms): (Vec<FloatColor>, Vec<Vec<HistColor>>) =
            self.0.into_iter().map(|node| (node.avg, node.histogram)).unzip();
        let histogram: Vec<HistColor> = histograms.iter().flat_map(|h| h.iter().cloned()).collect();
        for _ in 0..num_iterations {
            colors = kmeans.step(colors, &histogram);
        }
        let mut histograms: Vec<Vec<HistColor>> = (0..colors.len()).map(|_| Vec::new()).collect();
        let map = ColorMap::from_float_colors(colors);
        for color in histogram {
            histograms[map.find_nearest(color.color)].push(color);
        }
        Quantizer(histograms.into_iter().map(|h| QuantizerNode::new(h)).collect())
    }
}
