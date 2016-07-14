use std::collections::HashMap;

pub struct Histogram {
    data: HashMap<u32, usize>,
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram { data: HashMap::new() }
    }

    pub fn feed(&mut self, pixels: &[u32]) {
        for pixel in pixels {
            let count = self.data.entry(*pixel).or_insert(0);
            *count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_duplicates() {
        let mut hist = super::Histogram::new();
        hist.feed(&[0xaabbccffu32, 0x00ff00ff, 0x330088ff, 0x00ff00ff]);
        assert_eq!(*hist.data.get(&0x00ff00ff).unwrap(), 2usize);
    }
}
