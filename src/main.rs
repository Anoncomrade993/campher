mod Utils;
use image::GenericImageView;
use Utils::helpers::Block;

fn main() {
    let (w,h,vec) = read_image("./assets/specimen_1.jpg");
    let mut cvec = vec.clone();
    println!("{} * {}",w,h);
    println!("{:?}",&cvec[..10]);
    
    let d = 2u32;
    let res Block::rotate(&mut cvec,d)
    println!("{:?}",&res[..10]);
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