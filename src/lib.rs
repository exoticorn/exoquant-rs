mod color;
mod histogram;
mod quantizer;
pub mod colormap;
mod remapper;

pub use color::Color;
pub use histogram::Histogram;
pub use quantizer::create_palette;
pub use remapper::{Remapper, RemapperNoDither, RemapperOrdered};
