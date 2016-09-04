use super::*;

pub struct TestImage {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

pub fn test_image() -> TestImage {
    TestImage {
        pixels: (0..65536)
            .map(|i| {
                let x = i & 255;
                let y = i >> 8;
                Color::new(x as u8, y as u8, 0, 255)
            })
            .collect(),
        width: 256,
        height: 256,
    }
}
