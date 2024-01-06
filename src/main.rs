mod palette;

use image::{io::Reader as ImageReader, GenericImage, GenericImageView, ImageBuffer, ImageError};
use palette::{AverageSlice, SliceType};

fn main() -> Result<(), ImageError> {
    let image_path = "image.png".to_string();
    let img: image::DynamicImage = ImageReader::open(image_path)?.decode()?;
    let slices = palette::slice_image(&img, 4, SliceType::Vertical);

    let mut average = Vec::new();
    for slice in slices {
        average.push(palette::dominant_colour(slice))
    }

    let mut imgbuf = ImageBuffer::new(img.width(), img.height());

    for average_slice in average {
        let x = average_slice.bounds.0;
        let y = average_slice.bounds.1;
        let width = average_slice.bounds.2;
        let height = average_slice.bounds.3;

        for mut pixel in imgbuf.sub_image(x, y, width, height).pixels() {
            pixel.2 = average_slice.colour.clone();
            pixel.2 .0[3] = 0;
        }
    }

    imgbuf.save("out.png")
}
