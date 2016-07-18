use ::color::Color;
use ::color::FloatColor;

pub struct ColorMap {
    kdtree: KDNode,
    neighbor_distance: Vec<f64>,
    colors: Vec<FloatColor>,
}

struct KDNode {
    mid_point: FloatColor,
    index: usize,
    normal: FloatColor,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

struct KDNearest {
    index: usize,
    distance: f64,
}

impl KDNode {
    fn new(mut indices: Vec<usize>, colors: &[FloatColor]) -> KDNode {
        assert!(indices.len() > 0);
        let mut sum = FloatColor::default();
        let mut sum2 = FloatColor::default();
        for i in &indices {
            let c = colors[*i];
            sum += c;
            sum2 += c * c;
        }
        let var = sum2 - sum * sum * (1.0 / indices.len() as f64);
        let normal = if var.r > var.g && var.r > var.b && var.r > var.a {
            FloatColor {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        } else if var.g > var.b && var.g > var.a {
            FloatColor {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.0,
            }
        } else if var.b > var.a {
            FloatColor {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 0.0,
            }
        } else {
            FloatColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        };
        indices.sort_by(|a, b|
            colors[*a].dot(&normal).partial_cmp(&colors[*b].dot(&normal)).unwrap()
        );
        let i = indices.len() / 2;
        let left = if i > 0 {
            Some(Box::new(KDNode::new(indices[0..i].into(), colors)))
        } else {
            None
        };
        let right = if i + 1 < indices.len() {
            Some(Box::new(KDNode::new(indices[(i + 1)..].into(), colors)))
        } else {
            None
        };
        KDNode {
            mid_point: colors[indices[i]],
            index: indices[i],
            normal: normal,
            left: left,
            right: right,
        }
    }

    fn find_nearest(&self,
                    needle: FloatColor,
                    mut limit: f64,
                    ignore_index: usize)
                    -> Option<KDNearest> {
        let mut result = None;

        let diff = needle - self.mid_point;
        let distance = diff.dot(&diff).sqrt();

        if distance < limit && self.index != ignore_index {
            limit = distance;
            result = Some(KDNearest {
                index: self.index,
                distance: distance,
            })
        }

        let dot = diff.dot(&self.normal);
        if dot <= 0.0 {
            if let Some(ref left) = self.left {
                if let Some(nearest) = left.find_nearest(needle, limit, ignore_index) {
                    limit = nearest.distance;
                    result = Some(nearest);
                }
            }

            if -dot < limit {
                if let Some(ref right) = self.right {
                    if let Some(nearest) = right.find_nearest(needle, limit, ignore_index) {
                        result = Some(nearest);
                    }
                }
            }
        } else {
            if let Some(ref right) = self.right {
                if let Some(nearest) = right.find_nearest(needle, limit, ignore_index) {
                    limit = nearest.distance;
                    result = Some(nearest);
                }
            }

            if dot < limit {
                if let Some(ref left) = self.left {
                    if let Some(nearest) = left.find_nearest(needle, limit, ignore_index) {
                        result = Some(nearest);
                    }
                }
            }
        }

        result
    }
}

impl ColorMap {
    pub fn new(colors: &[Color]) -> ColorMap {
        let float_colors: Vec<_> = colors.iter().map(|c| c.into()).collect();
        Self::from_float_colors(float_colors)
    }

    pub fn from_float_colors(colors: Vec<FloatColor>) -> ColorMap {
        let kdtree = KDNode::new((0..colors.len()).collect(), &colors);
        let neighbor_distance = colors.iter()
            .enumerate()
            .map(|(i, c)| {
                if let Some(nearest) = kdtree.find_nearest(*c, ::std::f64::MAX, i) {
                    nearest.distance
                } else {
                    ::std::f64::MAX
                }
            })
            .collect();
        ColorMap {
            kdtree: kdtree,
            neighbor_distance: neighbor_distance,
            colors: colors,
        }
    }

    pub fn find_nearest(&self, color: &Color) -> usize {
        self.find_nearest_float(color.into())
    }

    pub fn find_nearest_float(&self, color: FloatColor) -> usize {
        if let Some(nearest) = self.kdtree.find_nearest(color, ::std::f64::MAX, ::std::usize::MAX) {
            nearest.index
        } else {
            0
        }
    }

    pub fn neighbor_distance(&self, index: usize) -> f64 {
        self.neighbor_distance[index]
    }

    pub fn float_color(&self, index: usize) -> FloatColor {
        self.colors[index]
    }
}