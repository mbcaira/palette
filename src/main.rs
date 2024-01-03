mod palette;

use image::{io::Reader as ImageReader, ImageError};
use palette::{AverageSlice, SliceType};

fn main() -> Result<(), ImageError> {
    let image_path = "image.png".to_string();
    let img: image::DynamicImage = ImageReader::open(image_path)?.decode()?;
    let slices = palette::slice_image(&img, 4, SliceType::Vertical);

    let mut average = Vec::new();
    for slice in slices {
        average.push(palette::dominant_colour(slice))
    }

    println!("{:?}", average);
    Ok(())
}
