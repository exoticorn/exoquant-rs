use super::*;
use optimizer::Optimizer;

struct QuantizerNode {
    histogram: Vec<ColorCount>, // a histogram of the colors represented by this node
    avg: Vec4<LabColor>,        // the average color of this node
    vdif: f32,                  // the improvement to the total variance when splitting this node
    split: usize,               // the best index to split this node at
}

impl QuantizerNode {
    fn new(mut histogram: Vec<ColorCount>) -> QuantizerNode {
        // First calculate the color average and variance over the histogram
        let mut n = 0usize;
        let mut fsum = Vec4::<LabColor>::zero();
        let mut fsum2 = Vec4::<LabColor>::zero();

        for entry in &histogram {
            let c = entry.color.to_vec4();
            n += entry.count;
            fsum += c * entry.count as f32;
            fsum2 += c * c * entry.count as f32;
        }

        if n == 0 {
            return QuantizerNode {
                histogram: histogram,
                avg: Vec4::zero(),
                vdif: 0.0,
                split: 0,
            };
        }

        let avg = fsum * (1.0 / n as f32);
        let vc = fsum2 - fsum * avg;
        let v = vc.x + vc.y + vc.z + vc.w;

        // Next sort histogram by the channel with the largest variance
        if vc.x > vc.y && vc.x > vc.z && vc.x > vc.w {
            histogram.sort_by(|a, b| a.color.l.partial_cmp(&b.color.l).unwrap());
        } else if vc.y > vc.z && vc.y > vc.w {
            histogram.sort_by(|a, b| a.color.a.partial_cmp(&b.color.a).unwrap());
        } else if vc.z > vc.w {
            histogram.sort_by(|a, b| a.color.b.partial_cmp(&b.color.b).unwrap());
        } else {
            histogram.sort_by(|a, b| a.color.alpha.partial_cmp(&b.color.alpha).unwrap());
        }

        // Determine primary vector of distribution in the histogram
        let mut dir = Vec4::<LabColor>::zero();
        for entry in &histogram {
            let mut tmp = (entry.color.to_vec4() - avg) * entry.count as f32;
            if tmp.dot(dir) < 0.0 {
                tmp *= -1.0;
            }
            dir += tmp;
        }

        dir *= {
            let s = dir.abs();
            if s < 0.000000001 {
                1.0f32
            } else {
                1.0 / s
            }
        };

        // Now sort histogram by primary vector
        histogram.sort_by(|a, b| {
            a.color
                .to_vec4()
                .dot(dir)
                .partial_cmp(&b.color.to_vec4().dot(dir))
                .unwrap()
        });

        // Find split index that results in lowest total variance
        let mut sum = Vec4::<LabColor>::zero();
        let mut sum2 = Vec4::<LabColor>::zero();
        let mut vdif = -v;
        let mut n2 = 0;
        let mut split = 0usize;
        for (i, entry) in histogram.iter().enumerate() {
            let c = entry.color.to_vec4();
            n2 += entry.count;
            sum += c * entry.count as f32;
            sum2 += c * c * entry.count as f32;

            if n2 < n {
                let tmp = sum2 - sum * sum * (1.0 / n2 as f32);
                let dif_sum = fsum - sum;
                let tmp2 = (fsum2 - sum2) - dif_sum * dif_sum * (1.0 / (n - n2) as f32);
                let nv = tmp.x + tmp.y + tmp.z + tmp.w + tmp2.x + tmp2.y + tmp2.z + tmp2.w;
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

/// The main color quantizer state.
///
/// The Quantizer is used to find a palette of colors that represent the colors in an input
/// Histogram as well as possible.
///
/// To use it you first create a new Quantizer instance using `Quantizer::new`, then call
/// `quantizer.step()` until `quantizer.num_colors()` reaches your target color count, then
/// call `quantizer.colors()` to get your final palette `Vec` of `Color`s.
///
/// Optionally you can call `quantizer.optimize()` between calls to `quantizer.step()` to
/// run K-Means optimizations during quantization.
///
/// If you don't intend to do K-Means optimzations during quantization, you can also just
/// use the shortcut function `Quantizer::create_palette`.
///
/// # Examples
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// # let histogram: Histogram = image.pixels.iter().cloned().collect();
/// # let colorspace = SimpleColorSpace::default();
/// let mut quantizer = Quantizer::new(&histogram, &colorspace);
/// while quantizer.num_colors() < 256 {
///   quantizer.step();
/// }
/// let palette = quantizer.colors(&colorspace);
/// ```
///
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// # let histogram: Histogram = image.pixels.iter().cloned().collect();
/// # let colorspace = SimpleColorSpace::default();
/// let palette = Quantizer::create_palette(&histogram, &colorspace, 256);
/// ```
pub struct Quantizer(Vec<QuantizerNode>);

impl Quantizer {
    /// Create a new Quantizer state for the given histogram.
    pub fn new(histogram: &::histogram::Histogram) -> Quantizer {
        let hist = histogram.to_color_counts();
        Quantizer(vec![QuantizerNode::new(hist)])
    }

    /// A shortcut function to directly create a palette from a histogram.
    pub fn create_palette(histogram: &::histogram::Histogram, num_colors: usize) -> Vec<Color> {
        let mut quantizer = Self::new(histogram);
        while quantizer.num_colors() < num_colors {
            quantizer.step();
        }
        quantizer.colors()
    }

    /// Returns the current number of colors in this Quantizer state.
    ///
    /// This starts off at 1 and increases by 1 for each call to `quantizer.step()`.
    pub fn num_colors(&self) -> usize {
        self.0.len()
    }

    /// Run one quantization step which increases the `num_colors()` by one.
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

    /// Returns colors the current Quantizer state represents..
    pub fn colors(&self) -> Vec<Color> {
        self.0
            .iter()
            .map(|node| node.avg.to_color().into())
            .collect()
    }

    /// Run a number of K-Means iteration on the current quantizer state.
    ///
    /// This can improve the quality of the final palette by a certain amount,
    /// but note that this is a fairly expensive operation. You might find it
    /// sufficient to only run optimizations on the final palette, in which case
    /// `optimizer.optimize_palette` is slightly less expensive.
    ///
    /// # Examples
    /// ```
    /// # use exoquant::*;
    /// # let image = testdata::test_image();
    /// # let histogram: Histogram = image.pixels.iter().cloned().collect();
    /// # let colorspace = SimpleColorSpace::default();
    /// let optimizer = optimizer::KMeans;
    /// let mut quantizer = Quantizer::new(&histogram, &colorspace);
    /// while quantizer.num_colors() < 256 {
    ///   quantizer.step();
    ///   if quantizer.num_colors() % 32 == 0 {
    ///     quantizer = quantizer.optimize(&optimizer, 4);
    ///   }
    /// }
    /// let palette = quantizer.colors(&colorspace);
    /// ```
    pub fn optimize(self, optimizer: &Optimizer, num_iterations: usize) -> Quantizer {
        if optimizer.is_noop() {
            return self;
        }
        let (mut colors, histograms): (Vec<LabColor>, Vec<Vec<ColorCount>>) = self
            .0
            .into_iter()
            .map(|node| (node.avg.to_color(), node.histogram))
            .unzip();
        let histogram: Vec<ColorCount> =
            histograms.iter().flat_map(|h| h.iter().cloned()).collect();
        for _ in 0..num_iterations {
            colors = optimizer.step(colors, &histogram);
        }
        let mut histograms: Vec<Vec<ColorCount>> = (0..colors.len()).map(|_| Vec::new()).collect();
        let map = ColorMap::from_float_colors(colors);
        for color in histogram {
            histograms[map.find_nearest(color.color)].push(color);
        }
        Quantizer(
            histograms
                .into_iter()
                .map(|h| QuantizerNode::new(h))
                .collect(),
        )
    }
}
