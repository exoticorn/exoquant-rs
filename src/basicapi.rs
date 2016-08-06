use super::*;

pub fn convert_to_indexed<T: Ditherer>(image: &[Color],
                                       width: usize,
                                       num_colors: usize,
                                       ditherer: &T)
                                       -> (Vec<Color>, Vec<u8>) {
    let colorspace = SimpleColorSpace::default();

    let hist = image.iter().cloned().collect();

    let palette = generate_palette(&hist, &colorspace, num_colors);

    let palette = optimize_palette(&colorspace, &palette, &hist, 8);

    let remapper = Remapper::new(&palette, &colorspace, ditherer);
    let image: Vec<_> = remapper.remap8(image, width);

    sort_palette(&palette, &image)
}

pub fn generate_palette<T: ColorSpace>(hist: &Histogram,
                                       colorspace: &T,
                                       num_colors: usize)
                                       -> Vec<Color> {
    let mut quantizer = Quantizer::new(hist, colorspace);
    let kmeans_step = (num_colors as f64).sqrt().round() as usize;
    while quantizer.num_colors() < num_colors {
        quantizer.step();
        if quantizer.num_colors() % kmeans_step == 0 {
            quantizer = quantizer.do_kmeans_optimization(2);
        }
    }
    quantizer.colors(colorspace)
}
