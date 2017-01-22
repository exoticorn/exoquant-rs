use super::*;

/// A data structure for fast nearest color lookups in a palette.
pub struct ColorMap {
    kdtree: KDNode,
    neighbor_distance: Vec<f64>,
    neighbors: Vec<Vec<usize>>,
    colors: Vec<Colorf>,
}

struct KDNode {
    mid_point: Colorf,
    index: usize,
    normal: Colorf,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

struct KDNearest {
    index: usize,
    distance: f64,
}

impl KDNode {
    fn new(mut indices: Vec<usize>, colors: &[Colorf]) -> KDNode {
        assert!(indices.len() > 0);
        let mut sum = Colorf::zero();
        let mut sum2 = Colorf::zero();
        for i in &indices {
            let c = colors[*i];
            sum += c;
            sum2 += c * c;
        }
        let var = sum2 - sum * sum * (1.0 / indices.len() as f64);
        let normal = if var.r > var.g && var.r > var.b && var.r > var.a {
            Colorf {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        } else if var.g > var.b && var.g > var.a {
            Colorf {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.0,
            }
        } else if var.b > var.a {
            Colorf {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 0.0,
            }
        } else {
            Colorf {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        };
        indices.sort_by(|a, b| {
            colors[*a].dot(&normal).partial_cmp(&colors[*b].dot(&normal)).unwrap()
        });
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
                    needle: Colorf,
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

fn occludes(origin: Colorf, occluder: Colorf, target: Colorf) -> bool {
    let dir = occluder - origin;
    dir.dot(&dir) * 0.5 <= (target - origin).dot(&dir)
}

impl ColorMap {
    /// Create a `ColorMap` from a slice of `Color`s.
    pub fn new<T: ColorSpace>(colors: &[Color], colorspace: &T) -> ColorMap {
        let float_colors: Vec<_> =
            colors.iter().map(|c| colorspace.output_to_quantization((*c).into())).collect();
        Self::from_float_colors(float_colors)
    }

    /// Create a `ColorMap` from float colors.
    pub fn from_float_colors(colors: Vec<Colorf>) -> ColorMap {
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
        let neighbors = colors.iter()
            .enumerate()
            .map(|(i, &c)| {
                let mut vec = Vec::new();
                for (j, &c2) in colors.iter().enumerate() {
                    if i != j && vec.iter().all(|&k| !occludes(c, colors[k], c2)) {
                        vec.retain(|&k| !occludes(c, c2, colors[k]));
                        vec.push(j);
                    }
                }
                vec
            })
            .collect();
        ColorMap {
            kdtree: kdtree,
            neighbor_distance: neighbor_distance,
            neighbors: neighbors,
            colors: colors,
        }
    }

    /// Returns the index of the nearest color in the palette.
    pub fn find_nearest(&self, color: Colorf) -> usize {
        if let Some(nearest) = self.kdtree.find_nearest(color, ::std::f64::MAX, ::std::usize::MAX) {
            nearest.index
        } else {
            0
        }
    }

    /// Returns the distance to the closest neighbor color of a palette entry given by index.
    pub fn neighbor_distance(&self, index: usize) -> f64 {
        self.neighbor_distance[index]
    }

    /// Returns the list of neighbors (as indices) for a palette color given by index.
    pub fn neighbors(&self, index: usize) -> &[usize] {
        &self.neighbors[index]
    }

    /// Returns the palette color for the given index.
    pub fn float_color(&self, index: usize) -> Colorf {
        self.colors[index]
    }

    /// Returns the number of colors in the palette.
    pub fn num_colors(&self) -> usize {
        self.colors.len()
    }
}
