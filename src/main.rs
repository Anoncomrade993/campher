mod Utils;
use image::{GenericImageView, ImageBuffer, Rgb};


fn main() {
    
 }

fn read_image(path: &str) -> (u32,u32,Vec<u8>){
  let im = image::open(path).unwrap();
  let (w, h) = im.dimensions();
  let mut vec = im.pixels().map(|_,_,pixel|{
    let rgba = pixel.to_rgba();
    vec![rgba[0], rgba[1], rgba[2], rgba[3]]
  }).collect();
  (w,h,vec)
}