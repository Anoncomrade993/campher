use image::{open, GenericImageView, ImageResult, RgbaImage};

pub mod block {
    pub fn rotate(pixels: &mut Vec<u8>, width: u32, height: u32, d: u32) -> Vec<u8> {
        let n = pixels.len() as u32;
        let d = d % n;
        let mut rotated_pixels = pixels.clone();

        reverse(&mut rotated_pixels, 0, d - 1, width, height);
        reverse(&mut rotated_pixels, d, n - 1, width, height);
        reverse(&mut rotated_pixels, 0, n - 1, width, height);

        rotated_pixels
    }

    fn reverse(pixels: &mut Vec<u8>, start: u32, end: u32, width: u32, height: u32) {
        let mut i = start;
        let mut j = end;

        while i < j {
            let idx1 = (i / 4) as usize;
            let idx2 = (j / 4) as usize;
            pixels.swap(idx1, idx2);
            i += 4;
            j -= 4;
        }
    }
}

/// Reads a photo from a file and returns its dimensions and RGBA pixels
fn read_photo(file_path: &str) -> ImageResult<(u32, u32, Vec<u8>)> {
    let img = open(file_path)?;
    let (width, height) = img.dimensions();
    let pixels = img.to_rgba8().into_raw();
    println!("read image");
    Ok((width, height, pixels))
}

/// Writes a photo to a file with the given dimensions and RGBA pixels
fn write_photo(
    file_path: &str,
    width: u32,
    height: u32,
    pixels: Vec<u8>,
) -> Result<(), image::ImageError> {
    let img = RgbaImage::from_raw(width, height, pixels).ok_or(image::ImageError::Parameter(
        image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ),
    ))?;
    img.save(file_path)?;
    println!("write image");
    Ok(())
}

fn main() {
    let input_path = "src/assets/test.jpeg";
    let output_path = "src/assets/output.png";

    let (width, height, mut pixels) = read_photo(input_path).expect("Failed to read image");

    let rotated_pixels = block::rotate(&mut pixels, width, height, 10);

    write_photo(output_path, width, height, rotated_pixels).expect("Failed to write rotated image");

    println!("Rotated image saved successfully!");
}
