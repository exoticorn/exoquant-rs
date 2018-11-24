use super::*;

/// A data structure for fast nearest color lookups in a palette.
pub struct ColorMap {
    kdtree: KDNode,
    neighbor_distance: Vec<f32>,
    neighbors: Vec<Vec<usize>>,
    colors: Vec<LabColor>,
}

struct KDNode {
    mid_point: LabColor,
    index: usize,
    normal: Vec4<LabColor>,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

struct KDNearest {
    index: usize,
    distance: f32,
}

impl KDNode {
    fn new(mut indices: Vec<usize>, colors: &[LabColor]) -> KDNode {
        assert!(indices.len() > 0);
        let mut sum = Vec4::<LabColor>::zero();
        let mut sum2 = Vec4::<LabColor>::zero();
        for i in &indices {
            let c = colors[*i].to_vec4();
            sum += c;
            sum2 += c * c;
        }
        let var = sum2 - sum * sum * (1.0 / indices.len() as f32);
        let normal = if var.x > var.y && var.x > var.z && var.x > var.w {
            Vec4::<LabColor>::new(1., 0., 0., 0.)
        } else if var.y > var.z && var.y > var.w {
            Vec4::<LabColor>::new(0., 1., 0., 0.)
        } else if var.z > var.w {
            Vec4::<LabColor>::new(0., 0., 1., 0.)
        } else {
            Vec4::<LabColor>::new(0., 0., 0., 1.)
        };
        indices.sort_by(|a, b| {
            colors[*a]
                .to_vec4()
                .dot(normal)
                .partial_cmp(&colors[*b].to_vec4().dot(normal))
                .unwrap()
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

    fn find_nearest(
        &self,
        needle: LabColor,
        mut limit: f32,
        ignore_index: usize,
    ) -> Option<KDNearest> {
        let mut result = None;

        let diff = needle.to_vec4() - self.mid_point.to_vec4();
        let distance = diff.abs();

        if distance < limit && self.index != ignore_index {
            limit = distance;
            result = Some(KDNearest {
                index: self.index,
                distance: distance,
            })
        }

        let dot = diff.dot(self.normal);
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

fn occludes(origin: LabColor, occluder: LabColor, target: LabColor) -> bool {
    let dir = occluder.to_vec4() - origin.to_vec4();
    dir.dot(dir) * 0.5 <= (target.to_vec4() - origin.to_vec4()).dot(dir)
}

impl ColorMap {
    /// Create a `ColorMap` from a slice of `Color`s.
    pub fn new(colors: &[Color]) -> ColorMap {
        let float_colors: Vec<_> = colors
            .iter()
            .map(|&c| XyzColor::from(SrgbColor::from(c)).into())
            .collect();
        Self::from_float_colors(float_colors)
    }

    /// Create a `ColorMap` from float colors.
    pub fn from_float_colors(colors: Vec<LabColor>) -> ColorMap {
        let kdtree = KDNode::new((0..colors.len()).collect(), &colors);
        let neighbor_distance = colors
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if let Some(nearest) = kdtree.find_nearest(*c, ::std::f32::MAX, i) {
                    nearest.distance
                } else {
                    ::std::f32::MAX
                }
            }).collect();
        let neighbors = colors
            .iter()
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
            }).collect();
        ColorMap {
            kdtree: kdtree,
            neighbor_distance: neighbor_distance,
            neighbors: neighbors,
            colors: colors,
        }
    }

    /// Returns the index of the nearest color in the palette.
    pub fn find_nearest(&self, color: LabColor) -> usize {
        if let Some(nearest) = self
            .kdtree
            .find_nearest(color, ::std::f32::MAX, ::std::usize::MAX)
        {
            nearest.index
        } else {
            0
        }
    }

    /// Returns the distance to the closest neighbor color of a palette entry given by index.
    pub fn neighbor_distance(&self, index: usize) -> f32 {
        self.neighbor_distance[index]
    }

    /// Returns the list of neighbors (as indices) for a palette color given by index.
    pub fn neighbors(&self, index: usize) -> &[usize] {
        &self.neighbors[index]
    }

    /// Returns the palette color for the given index.
    pub fn float_color(&self, index: usize) -> LabColor {
        self.colors[index]
    }

    /// Returns the number of colors in the palette.
    pub fn num_colors(&self) -> usize {
        self.colors.len()
    }
}
