extern crate exoquant;

fn main() {
    let mut hist = exoquant::Histogram::new();
    hist.feed(&[0xaabbccffu32, 0x00ff00ff, 0x330088ff, 0x00ff00ff]);
    println!("Hello, World!");
}
