use image::{DynamicImage, GenericImageView, Rgba, SubImage};

#[derive(Debug, PartialEq, Eq)]
pub enum SliceType {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct AverageSlice {
    pub colour: Rgba<u8>,
    pub bounds: (u32, u32, u32, u32),
}

pub fn slice_image(
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
                println!("{}", dimensions.1);
                image_slices.push(img.view(i as u32 * size, 0, size, dimensions.1));
            }
            SliceType::Horizontal => {
                let size = dimensions.1 / num_sections as u32;
                image_slices.push(img.view(0, i as u32 * size, size, dimensions.0));
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
        slice.bounds().2,
        slice.bounds().3,
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
pub fn knn_colour(slice: &AverageSlice, k: usize) -> AverageSlice {
    todo!()
}
