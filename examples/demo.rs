extern crate exoquant;

mod png;

use exoquant::*;
use exoquant::remapper::*;

fn main() {
    println!("Loading PNG");
    let (input_image, width, height) = png::load("test.png");

    let colorspace = SimpleColorSpace::default();

    println!("Building histogram");
    let mut hist = exoquant::Histogram::new();
    hist.extend(input_image.iter().cloned());

    println!("Generating palette");
    let mut quantizer = exoquant::Quantizer::new(&hist, &colorspace);
    while quantizer.num_colors() < 256 {
        quantizer.step();
        if quantizer.num_colors() % 16 == 0 {
            quantizer = quantizer.do_kmeans_optimization(2);
        }
    }
    let palette = quantizer.colors(&colorspace);

    println!("Optimize palette (k-means)");
    let palette = exoquant::optimize_palette(&colorspace, palette, &hist, 8);

    println!("Remapping image to palette");
    let remapper = Remapper::new(&palette, &colorspace, DithererFloydSteinberg::checkered());
    let image: Vec<_> = remapper.remap8(&input_image, width);

    println!("Saving PNG");
    png::save("out.png", &palette, &image, width, height);

    println!("done!");
}
