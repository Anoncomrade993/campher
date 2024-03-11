mod _;
use _::helpers::{
  get_pixels,
  base64_to_image,
  image_to_base64,
  vec_to_image,
  Block::{rotate_vec}
}


fn main (){
  let s = "".to_string()
  let res = base64_to_image(s)
  println!("{:?}",res)
}