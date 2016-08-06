extern crate exoquant;

mod png;

use exoquant::*;
use std::env;

// usage: cargo run --release --demo highlevel <in.png> <out.png> <num_colors>
fn main() {
    let mut args = env::args();
    args.next();
    let in_name = args.next().unwrap();
    let out_name = args.next().unwrap();
    let num_colors: usize = args.next().unwrap().parse().unwrap();

    let (input_image, width, height) = png::load(&in_name);

    let (palette, out_image) = convert_to_indexed(&input_image,
                                                  width,
                                                  num_colors,
                                                  &DithererFloydSteinberg::checkered());

    png::save(&out_name, &palette, &out_image, width, height);
}
