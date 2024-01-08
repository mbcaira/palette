mod palette;

use palette::{generate_palette, AveragingMethod, SliceType, SlicingConfig};

fn main() {
    generate_palette(
        AveragingMethod::Dominant,
        "image.png".to_string(),
        10,
        "out.png".to_string(),
        Some(SlicingConfig {
            direction: SliceType::Horizontal,
            cluster_size: None,
        }),
    )
}
