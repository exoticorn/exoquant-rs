extern crate exoquant;

use exoquant::Color;

fn main() {
    let mut hist = exoquant::Histogram::new();
    hist.extend([0xaabbccffu32, 0x00ff00ff, 0x330088ff, 0x00ff00ff].iter().map(|c| Color(*c)));
    println!("Hello, World!");
}
