use super::*;

pub struct TestImage {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

pub fn test_image() -> TestImage {
    TestImage {
        pixels: (0..65536)
            .map(|_| Color::new(0, 0, 0, 255))
            .collect(),
        width: 256,
        height: 256,
    }
}
