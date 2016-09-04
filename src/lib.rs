//! Exoquant is a very high quality image quantization library featuring code for basic
//! color quantization, K-Means palette optimization and remapping and dithering with
//! Floyd-Steinberg and ordered ditherers.
//!
//! This version of the library is a much improved rewrite of a C library of the same name written
//! back in 2004.
//!
//! # Basic API:
//! For simple use cases, there is a convenience function that simply takes true color
//! image data + a few options as input and returns the palette and indexed image data as
//! output:
//!
//! ```
//! use exoquant::*;
//! let image = testdata::test_image();
//!
//! let (palette, indexed_data) = convert_to_indexed(&image.pixels, image.width, 256,
//!   &optimizer::KMeans, &ditherer::FloydSteinberg::new());
//! ```
//!
//! # Low-Level API:
//! The low-level API gives you full control over the quantization workflow. It allows for
//! use-cases like:
//!
//! * only create a palette and do the remapping in your own custom code
//! * remap images to an existing palette or one created with a different library
//! * generating a single palette for multiple input images (or, say, frames of a GIF)
//! * implement your own custom ditherer (also usable with the basic API)
//!
//! Using the low-level API to quantize an image looks like this:
//!
//! ```
//! use exoquant::*;
//! use exoquant::optimizer::Optimizer;
//!
//! let image = testdata::test_image();
//!
//! let histogram = image.pixels.iter().cloned().collect();
//!
//! let colorspace = SimpleColorSpace::default();
//! let optimizer = optimizer::KMeans;
//! let mut quantizer = Quantizer::new(&histogram, &colorspace);
//! while quantizer.num_colors() < 256 {
//!   quantizer.step();
//!   // very optional optimization, !very slow!
//!   // you probably only want to do this every N steps, if at all.
//!   if quantizer.num_colors() % 64 == 0 {
//!     quantizer = quantizer.optimize(&optimizer, 4);
//!   }
//! }
//!
//! let palette = quantizer.colors(&colorspace);
//! // this optimization is more useful than the above and a lot less slow
//! let palette = optimizer.optimize_palette(&colorspace, &palette, &histogram, 16);
//!
//! let ditherer = ditherer::FloydSteinberg::new();
//! let remapper = Remapper::new(&palette, &colorspace, &ditherer);
//! let indexed_data = remapper.remap(&image.pixels, image.width);
//! ```

mod color;
mod histogram;
mod quantizer;
mod colormap;
mod remapper;
pub mod optimizer;
mod colorspace;
mod palettesort;
mod basicapi;
#[cfg(feature="random-sample")]
pub mod random_sample;
pub mod ditherer;
#[doc(hidden)]
pub mod testdata;

pub use basicapi::{convert_to_indexed, generate_palette};
pub use color::*;
pub use colormap::ColorMap;
pub use colorspace::{ColorSpace, SimpleColorSpace};
pub use histogram::*;
pub use palettesort::sort_palette;
pub use quantizer::Quantizer;
#[cfg(feature="random-sample")]
pub use random_sample::RandomSample;
pub use remapper::Remapper;
