mod palette;

use palette::{generate_palette, AveragingMethod, SliceType, SlicingConfig};
// TODO
// Convert this to an AWS Lambda
// User uploads image to bucket with params like sections and slice config
// Lambda is triggered on upload, sending image path with user params
// Output new image/palette
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
