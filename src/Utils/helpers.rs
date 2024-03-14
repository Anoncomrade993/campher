use image::{
 ImageFormat,GenericImageView,Image,ImageResult,GenericImage,Rgb,Rgb8,DynamicImage
  
};
use tokio::task;
use base64::{engine::general_purpose::STANDARD::{decode,encode},
Engine as _  
};


//first the json body with  image
//get pixels ✓
//block swap it ✓
//send back to image 
//convert to base64 again
//return to UI

pub fn get_pixels<'r>(src:String)-> Result<(Vec<u8>,ImageResult<Image>),image::error::Error> {
 let im = image::open(src)?;
  let vec :Vec<u8> = &im.pixels().map(|_,_,pixel|{
       let rgba = pixel.to_rgba();
       vec![rgba[0],rgba[1],rgba[2],rgba[3]]
   }).collect();
   
   Ok(vec,im)
}


pub fn vectors_to_image(pixel_data: Vec<u8>, width: u32, height: u32) -> Result<(), image::ImageError> {
let mut buffers = vec![];
    // Create the image buffer directly from the pixel data
    let raw_data: ImageBuffer<Rgb<u8>,Vec<u8>> = ImageBuffer::from_raw(width, height, pixel_data)?;
    
    let im = DynamicImage::ImageRgb8(&raw_data);
    im.write_to(&mut buffers,image::ImageOutputFormat::PNG)?;
    enc(&)
    Ok(buffers)
    // Return a success result
}


pub fn image_to_base64(src:ImageResult<Image>)-> Result<String,String>{
  let mut buf = vec![];
  src.write_to(&mut buf,ImageFormat::PNG);
  enc(&mut buf);
}

fn enc(src:&mut Vec<u8>)->Result<String,String>{
  match encode(&buf){
    Ok(data) => String::from(data),
    Err(_) => return Err("Error occurred while image_to_base64")
  }
}


pub struct Block{};
impl Block {
    pub async fn rotate(vec: &mut Vec<u8>, d: usize) -> &Vec<u8> {
        let n = vec.len();
        let d = d % n;
        let mut vec_clone = vec.clone();
        let (vec_clone, mid) = Self::reverse(&mut vec_clone, 0, d - 1).await;
        Self::reverse(&mut vec_clone, d, n - 1).await;
        Self::reverse(&mut vec_clone, 0, n - 1).await;
        *vec = vec_clone;
        vec
    }

    async fn reverse(vec: &mut Vec<u8>, start: usize, end: usize) -> Vec<u8> {
        let mut i = start;
        let mut j = end;
        while i < j {
            vec.swap(i, j);
            i += 1;
            j -= 1;
        }
        vec.clone()
    }
}

