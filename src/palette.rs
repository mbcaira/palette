use image::{
    io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, Rgba, SubImage,
};

const DEFAULT_CLUSTER_SIZE: usize = 4;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SliceType {
    Horizontal,
    Vertical,
}
pub enum AveragingMethod {
    Dominant,
    Cluster,
}
#[derive(Debug)]
pub struct AverageSlice {
    pub colour: Rgba<u8>,
    pub bounds: (u32, u32, u32, u32),
}

pub struct SlicingConfig {
    pub direction: SliceType,
    pub cluster_size: Option<usize>,
}

fn slice_image(
    img: &DynamicImage,
    num_sections: usize,
    slice_type: SliceType,
) -> Vec<SubImage<&DynamicImage>> {
    let dimensions = img.dimensions();

    let mut image_slices = Vec::new();

    for i in 0..num_sections {
        match slice_type {
            SliceType::Vertical => {
                let size = dimensions.0 / num_sections as u32;
                image_slices.push(img.view(i as u32 * size, 0, size, dimensions.1));
            }
            SliceType::Horizontal => {
                let size = dimensions.1 / num_sections as u32;
                image_slices.push(img.view(0, i as u32 * size, dimensions.0, size));
            }
        };
    }

    image_slices
}

pub fn dominant_colour(slice: SubImage<&DynamicImage>) -> AverageSlice {
    let mut average_r: u32 = 0;
    let mut average_g: u32 = 0;
    let mut average_b: u32 = 0;
    let mut average_a: u32 = 0;

    let mut slice_length = 0;
    for pixel in slice.pixels() {
        average_r += pixel.2[0] as u32;
        average_g += pixel.2[1] as u32;
        average_b += pixel.2[2] as u32;
        average_a += pixel.2[3] as u32;

        slice_length += 1;
    }
    let bounds = (
        slice.bounds().0,
        slice.bounds().1,
        slice.bounds().0 + slice.bounds().2,
        slice.bounds().1 + slice.bounds().3,
    );
    AverageSlice {
        colour: Rgba::<u8>::from([
            (average_r / slice_length as u32) as u8,
            (average_g / slice_length as u32) as u8,
            (average_b / slice_length as u32) as u8,
            (average_a / slice_length as u32) as u8,
        ]),
        bounds,
    }
}

#[allow(unused)]
pub fn knn_colour(slice: SubImage<&DynamicImage>, k: usize) -> AverageSlice {
    todo!()
}

pub fn generate_palette(
    method: AveragingMethod,
    image_path: String,
    num_sections: usize,
    output_path: String,
    slicing_config: Option<SlicingConfig>,
) {
    let img: image::DynamicImage = ImageReader::open(image_path)
        .expect("Could not open image.")
        .decode()
        .expect("Could not decode image.");

    let slice_type = match &slicing_config {
        Some(config) => config.direction,
        None => SliceType::Vertical,
    };

    let slices = slice_image(&img, num_sections, slice_type);
    let mut average = Vec::new();
    for slice in slices {
        match method {
            AveragingMethod::Dominant => average.push(dominant_colour(slice)),
            AveragingMethod::Cluster => {
                let cluster_size = match &slicing_config {
                    Some(config) => config.cluster_size.unwrap_or(DEFAULT_CLUSTER_SIZE),
                    None => DEFAULT_CLUSTER_SIZE,
                };
                average.push(knn_colour(slice, cluster_size))
            }
        }
        average.push(dominant_colour(slice))
    }

    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());
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

    imgbuf.save(output_path).expect("Could not save image");
}
