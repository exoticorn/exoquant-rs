extern crate lodepng;

use exoquant::Color;

pub fn load(filename: &str) -> (Vec<Color>, usize, usize) {
    let img = lodepng::decode32_file(filename).unwrap();
    (img.buffer.as_ref().iter().map(|c| Color::new(c.r, c.g, c.b, c.a)).collect(),
     img.width,
     img.height)
}

pub fn save(filename: &str, palette: &[Color], image: &[u8], width: usize, height: usize) {
    let mut state = lodepng::State::new();
    for color in palette {
        unsafe {
            lodepng::ffi::lodepng_palette_add(&mut state.info_png().color,
                                              color.r,
                                              color.g,
                                              color.b,
                                              color.a);
            lodepng::ffi::lodepng_palette_add(&mut state.info_raw(),
                                              color.r,
                                              color.g,
                                              color.b,
                                              color.a);
        }
    }
    state.info_png().color.bitdepth = 8;
    state.info_png().color.colortype = lodepng::ColorType::LCT_PALETTE;
    state.info_raw().bitdepth = 8;
    state.info_raw().colortype = lodepng::ColorType::LCT_PALETTE;

    state.encode_file(filename, image, width, height).unwrap();
}
