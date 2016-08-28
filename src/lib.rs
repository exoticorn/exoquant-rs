mod color;
mod histogram;
mod quantizer;
mod colormap;
pub mod remapper;
pub mod optimizer;
mod colorspace;
mod palettesort;
mod basicapi;
pub mod random_sample;
pub mod ditherer;
#[doc(hidden)]
pub mod testdata;

pub use basicapi::{convert_to_indexed, generate_palette};
pub use color::*;
pub use colormap::ColorMap;
pub use colorspace::{ColorSpace, SimpleColorSpace};
pub use histogram::*;
pub use optimizer::Optimizer;
pub use palettesort::sort_palette;
pub use quantizer::Quantizer;
pub use random_sample::RandomSample;
pub use remapper::Remapper;
pub use ditherer::Ditherer;
