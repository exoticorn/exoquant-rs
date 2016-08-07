extern crate exoquant;
extern crate simplesvg;

mod png;

use exoquant::*;
use simplesvg::{Color as ColorSvg, ColorNone, Fig, Attr, Svg, Trans};
use std::fs::File;
use std::io::Write;

fn render_palette(colors: &[Color]) -> Fig {
    let dots = colors.iter().map(|&c| Fig::Circle(c.r() as f32, c.g() as f32, 4.)).collect();
    Fig::Multiple(dots).styled(Attr::default().fill(ColorNone).stroke(ColorSvg(0, 128, 0)))
}

fn render_histogram(hist: &Histogram) -> Fig {
    let dots = hist.iter()
        .map(|(&col, &cnt)| Fig::Circle(col.r() as f32, col.g() as f32, (cnt as f32).sqrt()))
        .collect();
    Fig::Multiple(dots)
}

fn render_box(hist: &Histogram, palette: &[Color]) -> Fig {
    let border = Fig::Rect(0., 0., 256., 256.)
        .styled(Attr::default().fill(ColorNone).stroke(ColorSvg(128, 128, 128)));
    Fig::Multiple(vec![border, render_histogram(hist), render_palette(palette)])
}

fn main() {
    let (image, width, height) = png::load("baboon.png");
    let image: Vec<_> = image.iter()
        .map(|&c| {
            Color::rgba(c.r(),
                        c.b(),
                        ((c.r() as u32 + c.g() as u32) / 2) as u8,
                        c.a())
        })
        .collect();

    let ditherer = DithererFloydSteinberg::new();

    let (palette, out_image) = convert_to_indexed(&image, width, 256, &ditherer);

    png::save("weighted_orig.png", &palette, &out_image, width, height);

    let colorspace = SimpleColorSpace::default();
    let hist = image.iter().cloned().random_sample(1000. / (width * height) as f32).collect();
    let palette_noopt = generate_palette(&hist, &colorspace, 8);

    let palette_noweight = optimize_palette(&colorspace, &palette_noopt, &hist, 32);

    let remapper = Remapper::new(&palette_noweight, &colorspace, &ditherer);
    let out_image: Vec<_> = remapper.remap8(&image, width);

    png::save("weighted_8.png",
              &palette_noweight,
              &out_image,
              width,
              height);

    let mut diags = vec![render_box(&hist, &palette_noopt)];
    let mut palette_weight = palette_noopt.clone();
    for _ in 0..8 {
        palette_weight = optimize_palette_weighted(&colorspace, &palette_weight, &hist, 1);
        diags.push(render_box(&hist, &palette_weight));
    }

    let remapper = Remapper::new(&palette_weight, &colorspace, &ditherer);
    let out_image: Vec<_> = remapper.remap8(&image, width);

    png::save("weighted_8w.png",
              &palette_weight,
              &out_image,
              width,
              height);

    let num_diags = diags.len();
    let svg =
        Svg(diags.into_iter()
                .enumerate()
                .map(|(index, diag)| {
                    diag.transformed(Trans::default().translate(0., index as f32 * 300.).scale(2.5))
                })
                .collect(),
            640,
            768 * num_diags as u32);
    let mut file = File::create("weighted.svg").unwrap();
    write!(file, "{}", svg).unwrap();
}
