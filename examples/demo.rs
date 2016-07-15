extern crate exoquant;
extern crate lodepng;

use exoquant::Color;

fn main() {
    println!("Loading PNG");
    let input = lodepng::decode32_file("test.png").unwrap();

    println!("Building histogram");
    let mut hist = exoquant::Histogram::new();
    hist.extend(input.buffer.as_ref().iter().map(|c| Color::rgba(c.r, c.g, c.b, c.a)));

    println!("Generating palette");
    let palette = exoquant::create_palette(&hist, 256);

    let mut state = lodepng::State::new();
    for color in &palette {
        unsafe {
            lodepng::ffi::lodepng_palette_add(&mut state.info_png().color,
                                              color.r(),
                                              color.g(),
                                              color.b(),
                                              color.a());
            lodepng::ffi::lodepng_palette_add(&mut state.info_raw(),
                                              color.r(),
                                              color.g(),
                                              color.b(),
                                              color.a());
        }
    }
    state.info_png().color.bitdepth = 8;
    state.info_png().color.colortype = lodepng::ColorType::LCT_PALETTE;
    state.info_raw().bitdepth = 8;
    state.info_raw().colortype = lodepng::ColorType::LCT_PALETTE;

    println!("Remapping image to palette");
    let map = exoquant::colormap::ColorMap::new(&palette);
    let image: Vec<_> = input.buffer
        .as_ref()
        .iter()
        .map(|c| map.find_nearest(&Color::rgba(c.r, c.g, c.b, c.a)) as u8)
        .collect();

    println!("Saving PNG");
    state.encode_file("out.png", &image, input.width, input.height).unwrap();
    println!("done!");
}
