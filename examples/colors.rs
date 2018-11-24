extern crate exoquant;

use exoquant::{LabColor, SrgbColor, XyzColor};
use std::env;

fn main() {
    let args: Vec<f32> = env::args().skip(1).map(|v| v.parse().unwrap()).collect();
    let srgb = SrgbColor::new(args[0], args[1], args[2], args[3]);
    println!("sRGB: {:?}", srgb);
    let xyz: XyzColor = srgb.into();
    println!("XYZ: {:?}", xyz);
    let lab: LabColor = xyz.into();
    println!("L*ab: {:?}", lab);
    let xyz: XyzColor = lab.into();
    println!("XYZ: {:?}", xyz);
    let srgb: SrgbColor = xyz.into();
    println!("sRGB: {:?}", srgb);
}
