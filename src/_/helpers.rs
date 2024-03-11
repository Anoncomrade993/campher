use image::{ImageBuffer,Pixels};

use base64::{
Engine as _, engine::GeneralPurpose::STANDARD::{
decode,encode
  }
};

pub struct Block{};
//first the json body with base64 string of image
//second ,convert string to image 
//get pixels ✓
//block swap it ✓
//send back to image 
//convert to base64 again
//return to UI

pub fn get_pixels(im:&image::ImageBuffer<u8,Vec<u8>)-> Result<Vec<u8>,image::Error>{
  let vec :Vec<u8> = im.enumerate_pixels()
   .map(|_,_,pixel|{
     let rgba = pixel.to_rgba();
     vec![rgba[0],rgba[1],rgba[2],rgba[3]]
   }).collect();
   vec
}



pub fn base64_to_image(src :String)-> Result<Vec<u8>,String>{
  let mut dec = decode(src.as_str()).unwrap();
  match image::load_from_memory(&dec.as_slice()).unwrap(){
    Ok(img) => get_pixels(&img),
    Err(_) => return Err("Error Loading base64",e)
  }
}

pub fn vec_to_image(w:u32,h:u32,vec:Vec<u8>) -> Result<String,String>{
let res =  match image::Rgba::from_raw(w,h,&vec){
   Ok(_) => {
     res.save("./_image.png")
   format!("saved")
   }
   Err(_) => return Error("Error vec to image")
 }
    
}
pub fn image_to_base64(src:ImageBuffer)-> Result<&[u8],String>{
  let mut buf = vec![];
  src.write_to(&mut buf,ImageOutputFormat::PNG);
  match encode(&buf){
    Ok(data) => data
    Err(_) => return Err("Error occurred while image_to_base64")
  }
}


impl Block{
  pub fn rotate(vec:&mut Vec<u8>,d:usize)-> Vec<u8>{
    let n = vec.len() as usize;
    let d = d % vec.len() as usize;
    swap(&mut vec,0,d-1);
    swap(&mut vec,d,n-1);
    swap(&mut vec,0,n-1);
   vec
  }
  
   fn reverse(vec:&mut Vec<u8>,start:usize,end:usize) ->(Vec<u8>,usize,usize){
        let mid = (start + end )/2;
        if start >= end {
           return;
         }
       vec[start..=mid].reverse();
       vec[mid+1..=end].reverse();
       vec[start..=end].reverse();
     (vec, start, end,mid)
   }
  
   fn swap(vec:&mut Vec<u8>,start:usize,end,usize) -> (){
   
     let vect = Self::reverse(&mut vec,start,end);
     let mid = vect.3;
     let st = vect.1;
     let en = vect.2;
     
     for i in st..=mid {
       let j = (st + en -i);
       vect.0.swap(i,j);
     }
     Ok(())
   }
}
