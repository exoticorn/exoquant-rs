mod color;
mod histogram;
mod quantizer;
mod colormap;
mod remapper;
mod kmeans;
mod colorspace;
mod palettesort;
mod basicapi;

pub use basicapi::convert_to_indexed;
pub use color::{Color, FloatColor};
pub use colormap::ColorMap;
pub use colorspace::{ColorSpace, SimpleColorSpace};
pub use histogram::Histogram;
pub use kmeans::{optimize_palette, optimize_palette_weighted};
pub use palettesort::sort_palette;
pub use quantizer::Quantizer;
pub use remapper::{Remapper, Ditherer, DithererNone, DithererOrdered, DithererExperimentalOrdered,
                   DithererFloydSteinberg};
