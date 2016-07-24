mod color;
mod histogram;
mod quantizer;
pub mod colormap;
pub mod remapper;
mod kmeans;
pub mod colorspace;

pub use color::Color;
pub use histogram::Histogram;
pub use quantizer::Quantizer;
pub use kmeans::{optimize_palette, optimize_palette_weighted};
pub use colorspace::{ColorSpace, SimpleColorSpace};
