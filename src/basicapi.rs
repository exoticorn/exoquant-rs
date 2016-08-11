use super::*;

pub fn convert_to_indexed<D, K>(image: &[Color],
                                width: usize,
                                num_colors: usize,
                                kmeans: &K,
                                ditherer: &D)
                                -> (Vec<Color>, Vec<u8>)
    where D: Ditherer,
          K: KMeans
{
    let colorspace = SimpleColorSpace::default();

    let hist = image.iter().cloned().collect();

    let palette = generate_palette(&hist, &colorspace, kmeans, num_colors);

    let palette = kmeans.optimize_palette(&colorspace, &palette, &hist, 8);

    let remapper = Remapper::new(&palette, &colorspace, ditherer);
    let image: Vec<_> = remapper.remap8(image, width);

    sort_palette(&palette, &image)
}

pub fn generate_palette<C, K>(hist: &Histogram,
                              colorspace: &C,
                              kmeans: &K,
                              num_colors: usize)
                              -> Vec<Color>
    where C: ColorSpace,
          K: KMeans
{
    let mut quantizer = Quantizer::new(hist, colorspace);
    let kmeans_step = (num_colors as f64).sqrt().round() as usize;
    while quantizer.num_colors() < num_colors {
        quantizer.step();
        if quantizer.num_colors() % kmeans_step == 0 {
            quantizer = quantizer.do_kmeans_optimization(kmeans, 2);
        }
    }
    quantizer.colors(colorspace)
}
