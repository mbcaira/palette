use image::{io::Reader as ImageReader, GenericImageView, ImageError, Rgba};
fn main() {
    let _ = create_image_slices("image.png".to_string(), 4, true);
}

#[derive(Debug)]
struct ImageSlice {
    pixels: Vec<Rgba<u8>>,
    bounds: (u32, u32),
}

fn create_image_slices(
    image_path: String,
    num_of_slices: u32,
    is_vertically_sliced: bool,
) -> Result<(), ImageError> {
    let img = ImageReader::open(image_path)?.decode()?;
    let dimensions = img.dimensions();

    let slice_size = if is_vertically_sliced {
        dimensions.0 / num_of_slices
    } else {
        dimensions.1 / num_of_slices
    };

    let mut image_slices = Vec::new();
    for i in 0..num_of_slices {
        image_slices.push(ImageSlice {
            pixels: Vec::new(),
            bounds: (i * slice_size, (i + 1) * slice_size),
        })
    }

    println!("{:?}", image_slices);

    for pixel in img.pixels() {
        let coord = if is_vertically_sliced {
            pixel.0
        } else {
            pixel.1
        };
        let colour = pixel.2;
        for slice in &mut image_slices {
            if slice.bounds.0 <= coord && coord < slice.bounds.1 {
                slice.pixels.push(colour);
                break;
            }
        }
    }

    let mut average_slices: Vec<ImageSlice> = Vec::new();

    for slice in &image_slices {
        average_slices.push(dominant_colour_slice(slice))
    }

    println!("{:?}", average_slices);

    Ok(())
}

fn dominant_colour_slice(slice: &ImageSlice) -> ImageSlice {
    let mut average_r: u32 = 0;
    let mut average_g: u32 = 0;
    let mut average_b: u32 = 0;
    let mut average_a: u32 = 0;

    let slice_length: usize = slice.pixels.len();

    for pixel in &slice.pixels {
        average_r += pixel.0[0] as u32;
        average_g += pixel.0[1] as u32;
        average_b += pixel.0[2] as u32;
        average_a += pixel.0[3] as u32;
    }
    ImageSlice {
        pixels: vec![Rgba::<u8>::from([
            (average_r / slice_length as u32) as u8,
            (average_g / slice_length as u32) as u8,
            (average_b / slice_length as u32) as u8,
            (average_a / slice_length as u32) as u8,
        ])],
        bounds: slice.bounds,
    }
}

#[allow(unused)]
fn kmeans_colour_slice(slice: &[Rgba<u8>]) -> Rgba<u8> {
    todo!()
}
