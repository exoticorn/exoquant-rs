use super::*;
use optimizer::Optimizer;
use ditherer::Ditherer;

/// A convenience function to simply quantize an image with sensible defaults.
///
/// # Examples:
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// let (palette, indexed_data) = convert_to_indexed(&image.pixels, image.width, 256,
///   &optimizer::KMeans, &ditherer::FloydSteinberg::new());
/// ```
pub fn convert_to_indexed<D, O>(image: &[Color],
                                width: usize,
                                num_colors: usize,
                                optimizer: &O,
                                ditherer: &D)
                                -> (Vec<Color>, Vec<u8>)
    where D: Ditherer,
          O: Optimizer
{
    let colorspace = SimpleColorSpace::default();

    let hist = image.iter().cloned().collect();

    let palette = generate_palette(&hist, &colorspace, optimizer, num_colors);

    let palette = optimizer.optimize_palette(&colorspace, &palette, &hist, 8);

    let image = Remapper::new(&palette, &colorspace, ditherer).remap(image, width);

    sort_palette(&palette, &image)
}

/// A convenience function to just generate a palette from a historam with sensible defaults.
///
/// # Examples:
/// ```
/// # use exoquant::*;
/// # let image = testdata::test_image();
/// # let histogram = image.pixels.iter().cloned().collect();
/// let palette = generate_palette(&histogram, &SimpleColorSpace::default(), &optimizer::KMeans,
///   256);
/// ```
pub fn generate_palette<C, O>(hist: &Histogram,
                              colorspace: &C,
                              optimizer: &O,
                              num_colors: usize)
                              -> Vec<Color>
    where C: ColorSpace,
          O: Optimizer
{
    let mut quantizer = Quantizer::new(hist, colorspace);
    let kmeans_step = if num_colors > 64 {
        num_colors
    } else if num_colors <= 16 {
        1
    } else {
        (num_colors as f64).sqrt().round() as usize
    };
    while quantizer.num_colors() < num_colors {
        quantizer.step();
        if quantizer.num_colors() % kmeans_step == 0 {
            quantizer = quantizer.optimize(optimizer, 4);
        }
    }
    quantizer.colors(colorspace)
}
