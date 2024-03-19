mod utils;
use image::{ImageBuffer,GenericImageView};
use utils::helpers::block;

fn main() {
    let path = "src/assets/test.jpg";

    let (w, h, vec) = read_image(path);
    let mut cvec = vec.clone();
    println!("{} * {}", w, h);
    println!("length :: {}", cvec.len());
    println!("Pixels::\n{:?}", &cvec[0..10]);

    let d: usize = 2;
    caller(&mut cvec, d);
    //println!("{:?}", res);
}

fn caller(vec: &mut Vec<Vec<u8>>, d: usize) -> () {
    let res = block::rotate(vec, d);
    println!("Rotation::\n{:?}", &res[0..10]);
}




fn create_rgba_image(path:&str,width: u32, height: u32, data: Vec<Vec<u8>>) -> Result< ImageBuffer<Rgba<u8>, Vec<u8>>,image::error::Error> {
    // Create an ImageBuffer with RGBA colors
    let mut img = ImageBuffer::from_vec(width, height, vec![0; width * height * 4]);

    // Iterate over each pixel and set its color
    for (x, row) in data.iter().enumerate() {
        for (y, pixel) in row.iter().enumerate() {
            // Convert u8 values to f32 and scale to [0, 1]
            let color = (*pixel as f32) / 255.0;
            // Set RGBA pixel
            img.put_pixel(x as u32, y as u32, Rgba([
    data[y][x * 4] as f32 / 255.0,
    data[y][x * 4 + 1] as f32 / 255.0,
    data[y][x * 4 + 2] as f32 / 255.0,
    1.0,  // Keep alpha at 1.0 (fully opaque)
]));
 }
    }
    img.save(path)?;
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
