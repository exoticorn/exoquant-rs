use ::color::Color;

/// Sort neighboring colors in the image to be neighbors in the palette as well.
///
/// Fairly silly and useless, but makes the palette (especially of ordered dithered images)
/// look a lot more tidy.
pub fn sort_palette(palette: &Vec<Color>, image: &Vec<u8>) -> (Vec<Color>, Vec<u8>) {
    let num_colors = palette.len();
    let mut counts: Vec<usize> = (0..num_colors).map(|_| 0).collect();
    let mut neighbors: Vec<Vec<usize>> =
        (0..num_colors).map(|_| (0..num_colors).map(|_| 0).collect()).collect();
    let mut last_index = 0;
    for &index in image {
        let index = index as usize;
        counts[index] += 1;
        neighbors[last_index][index] += 1;
        neighbors[index][last_index] += 1;
        last_index = index;
    }
    let mut mapping = Vec::new();
    let mut best_index = 0;
    let mut best_count = 0;
    for (index, &count) in counts.iter().enumerate() {
        if count > best_count {
            best_index = index;
            best_count = count;
        }
    }
    mapping.push(best_index);
    let mut available: Vec<usize> = (0..num_colors).filter(|&i| i != best_index).collect();
    let mut prev_index = best_index;
    while available.len() > 0 {
        let mut best_index = available[0];
        let mut best_count = 0;
        for &index in &available {
            let count = neighbors[prev_index][index];
            if count > best_count {
                best_index = index;
                best_count = count;
            }
        }
        available.retain(|&i| i != best_index);
        mapping.push(best_index);
        prev_index = best_index;
    }
    let new_palette: Vec<Color> = mapping.iter().map(|&i| palette[i]).collect();
    let mut reverse_mapping: Vec<u8> = (0..palette.len()).map(|_| 0).collect();
    for (a, &b) in mapping.iter().enumerate() {
        reverse_mapping[b] = a as u8;
    }
    let new_image = image.iter().map(|&i| reverse_mapping[i as usize]).collect();
    (new_palette, new_image)
}
