extern crate exoquant;

mod png;

use std::env;
use exoquant::*;
use exoquant::optimizer::Optimizer;

// usage: cargo run --release --demo lowlevel <in.png> <out.png> <num_colors>
fn main() {
    let mut args = env::args();
    args.next();
    let in_name = args.next().unwrap();
    let out_name = args.next().unwrap();
    let num_colors: usize = args.next().unwrap().parse().unwrap();

    println!("Loading PNG");
    let (input_image, width, height) = png::load(&in_name);

    let colorspace = SimpleColorSpace::default();

    println!("Building histogram");
    let hist = input_image.iter().cloned().collect();

    let optimizer = optimizer::WeightedKMeans;

    println!("Generating palette");
    let mut quantizer = Quantizer::new(&hist, &colorspace);
    let kmeans_step = (num_colors as f64).sqrt().round() as usize;
    while quantizer.num_colors() < num_colors {
        quantizer.step();
        if quantizer.num_colors() % kmeans_step == 0 {
            quantizer = quantizer.optimize(&optimizer, 2);
        }
    }
    let palette = quantizer.colors(&colorspace);

    println!("Optimize palette (k-means)");
    let palette = optimizer.optimize_palette(&colorspace, &palette, &hist, 8);

    println!("Remapping image to palette");
    let ditherer = ditherer::FloydSteinberg::checkered();
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let image: Vec<_> = remapper.remap(&input_image, width);

    let (palette, image) = sort_palette(&palette, &image);

    println!("Saving PNG");
    png::save(&out_name, &palette, &image, width, height);

    println!("done!");
}
