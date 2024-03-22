mod utils;
use image::{open, GenericImageView, ImageResult, RgbaImage};
use utils::helpers::block;
fn main() {
    let path = "src/assets/test.jpeg";
    let output = "src/assets/check.png";
    let (w, h, p) = read_photo(path).expect("Error");
    let d: u32 = 4;
    let vec = caller(&p, d);
    let _ = write_photo(output, w, h, vec).unwrap();
}
fn caller(pix: &[u8], d: u32) -> Vec<u8> {
    let mut vec = pix.to_vec();
    let res = block::rotate(&mut vec, d);
    res
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
    ));
    img?.save(file_path)?;
    println!("write image");
    Ok(())
}
