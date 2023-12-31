use image::{io::Reader as ImageReader, GenericImageView, ImageError, Rgba};
fn main() {
    let _ = create_image_slices("image.png".to_string(), 4, true);
}

fn create_image_slices(
    image_path: String,
    num_of_slices: u32,
    is_vertically_sliced: bool,
) -> Result<(), ImageError> {
    let img = ImageReader::open(image_path)?.decode()?;
    let dimensions = img.dimensions();

    let slice_size = dimensions.0 / num_of_slices;

    let mut slices: Vec<Vec<Rgba<u8>>> = Vec::new();
    for _ in 0..num_of_slices {
        slices.push(Vec::new())
    }

    for pixel in img.pixels() {
        let coord = if is_vertically_sliced {
            pixel.0
        } else {
            pixel.1
        };
        let colour = pixel.2;

        for (slice_number, pixels) in &mut slices.iter_mut().enumerate() {
            if coord < (slice_number as u32 + 1) * slice_size {
                pixels.push(colour);
                break;
            }
        }
    }

    let mut average_slices: Vec<Rgba<u8>> = Vec::new();

    for slice in &slices {
        average_slices.push(average_image_slice(slice))
    }

    println!("{:?}", average_slices);
    Ok(())
}

fn average_image_slice(slice: &[Rgba<u8>]) -> Rgba<u8> {
    todo!()
}
