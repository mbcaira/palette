mod palette;

use image::{io::Reader as ImageReader, ImageBuffer, Rgba};
use palette::SliceType;

fn main() {
    let image_path = "image.png".to_string();
    let img: image::DynamicImage = ImageReader::open(image_path)
        .expect("Could not open image.")
        .decode()
        .expect("Could not decode image.");
    let slices = palette::slice_image(&img, 4, SliceType::Horizontal);

    let mut average = Vec::new();
    for slice in slices {
        average.push(palette::dominant_colour(slice))
    }

    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0])
    }

    for average_slice in average {
        let x1 = average_slice.bounds.0;
        let y1 = average_slice.bounds.1;
        let x2 = average_slice.bounds.2;
        let y2 = average_slice.bounds.3;

        for pixel in imgbuf.enumerate_pixels_mut() {
            let pixel_x = pixel.0;
            let pixel_y = pixel.1;

            let in_x = x1 <= pixel_x && pixel_x <= x2;
            let in_y = y1 <= pixel_y && pixel_y <= y2;
            if in_x && in_y {
                *pixel.2 = average_slice.colour;
            }
        }
    }

    imgbuf.save("out.png").expect("Could not save image");
}
