use ::color::FloatColor;
use ::color::Color;
use ::colorspace::ColorSpace;

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
            if s < 0.000000001 { 1.0 } else { 1.0 / s }
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

pub fn create_palette<T: ColorSpace>(histogram: &::histogram::Histogram,
                                     colorspace: &T,
                                     num_colors: usize)
                                     -> Vec<Color> {
    let palette = create_palette_hist_colors(histogram.to_hist_colors(colorspace), num_colors);
    palette.iter().map(|c| colorspace.from_float(*c)).collect()
}

fn create_palette_hist_colors(hist: Vec<HistColor>, num_colors: usize) -> Vec<FloatColor> {

    let mut nodes = vec![QuantizerNode::new(hist)];

    while nodes.len() < num_colors {
        let (new_node1, new_node2) = {
            let node = {
                let mut best_i = 0;
                let mut best_e = 0.0;
                for i in 0..nodes.len() {
                    if nodes[i].vdif >= best_e {
                        best_e = nodes[i].vdif;
                        best_i = i;
                    }
                }
                nodes.swap_remove(best_i)
            };
            let mut colors1 = node.histogram;
            let colors2 = colors1.split_off(node.split);
            (QuantizerNode::new(colors1), QuantizerNode::new(colors2))
        };
        nodes.push(new_node1);
        nodes.push(new_node2);
    }

    nodes.iter().map(|n| n.avg).collect()
}
