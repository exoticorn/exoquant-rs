extern crate exoquant;
extern crate lodepng;

use exoquant::Color;

fn main() {
    let input = lodepng::decode32_file("test.png").unwrap();

    let mut hist = exoquant::Histogram::new();
    hist.extend(input.buffer.as_ref().iter().map(|c| Color::rgba(c.r, c.g, c.b, c.a)));

    let palette = exoquant::create_palette(&hist, 256);

    let image: Vec<_> =
        palette.iter().map(|c| lodepng::RGBA::new(c.r(), c.g(), c.b(), c.a())).collect();
    lodepng::encode32_file("palette.png", &image, 16, 16).unwrap();
    println!("done!");
}
