mod utils;
use image::{ImageBuffer,GenericImageView};
use utils::helpers::block;

fn main() {
    let path = "src/assets/test.jpg";

    let (w, h, vec) = read_image(path);
    let mut cvec = vec.clone();
    let outpath = "src/assets/output.png";
    let res = caller(&mut cvec, d);
   let width = cvec[0].len();
   let height = cvec.len();
  let _ = create_image(output,width, height, res);
    
}

fn caller(vec: &mut Vec<Vec<u8>>, d: usize) -> () {
    let res = block::rotate(vec, d);
    res
}




/*fn create_image(path:&str,width: u32, height: u32, data: Vec<Vec<u8>>) -> Result<(),image::ImageError> {
    let vec = data.clone();
    let 1dvec :Vec<u8> = vec.iter().flat_map(|d|{vec![d]}).collect();
    let pic = image::RgbaImage::from_raw(width as usize, height as usize,1dvec);
    
    
}*/
fn create_image(path: &str, width: u32, height: u32, data: Vec<Vec<u8>>) -> Result<(), image::ImageError> {
    let mut 1dvec: Vec<u8> = Vec::new();

    for row in data {
        for pixel in row {
            1dvec.push(pixel);
        }
    }

    let pic = image::RgbaImage::from_raw(width as u32, height as u32, 1dvec)
        .ok_or(image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        )))?;

    pic.save(path)?;
    println!("saved picture");
    Ok(())
}
fn read_image(path: &str) -> (u32, u32, Vec<Vec<u8>>) {
    let im = image::open(path).unwrap();
    let (w, h) = im.dimensions();
    let vec: Vec<Vec<u8>> = im
        .pixels()
        .map(|(_, _, pixel)| {
            let rgba = pixel;
            vec![rgba[0], rgba[1], rgba[2], rgba[3]]
        })
        .collect();
    (w, h, vec)
}
