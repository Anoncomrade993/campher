/************************************************
Copyright (c) 2024 Dessxvii(Destiny Ifeanyi)
All rights reserved.

 This code is the intellectual property of Dessxvii(Destiny Ifeanyi). You are granted a non-exclusive, non-transferable, revocable license to use this code for personal or educational purposes only. Any unauthorized use, reproduction, or distribution of this code without the express permission of Dessxvii (Destiny Ifeanyi) is strictly prohibited.

This code is provided "as is" without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and non-infringement. In no event shall Dessxvii (Destiny Ifeanyi) be liable for any claim, damages, or other liability arising from the use of this code.

If you wish to use this code for commercial purposes or require a different licensing arrangement, please contact destinyifeanyi001@gmail.com.

Unauthorized copying or distribution of this code will be considered a violation of intellectual property rights and may result in legal action.
************************************************/

use base64::{
    engine::{general_purpose::STANDARD as st},
    Engine as _,
};
use image::GenericImageView;
use image::Pixel;
use pixelate::algorithms::lsb::LSB;


//read image pixels into Vector 
pub fn get_image_pixels(path: &str) -> Result<Vec<u8>, image::ImageError> {
    let img = image::open(path)?;
    let pixel_data: Vec<u8> = img
        .pixels()
        .flat_map(|(_, _, pixel)| {
            let rgba = pixel.to_rgba();
            vec![rgba[0], rgba[1], rgba[2], rgba[3]]
        })
        .collect();
    Ok(pixel_data)
}

//convert text to base64 string 
pub fn text_to_base64(s: &str) -> String {
    st.encode(s)
}
//base64 string to text (utf-8)
pub fn base64_to_text(s: String) -> String {
    st.decode(s) as String
}
//text to binary string
pub fn text_to_binary(text: String) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join("")
}
//binary string to text (utf-8)
pub fn binary_to_text(bin: String) -> String {
    bin.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| {
            let s: String = chunk.iter().collect();
            let byte: u8 = u8::from_str_radix(&s, 2).unwrap();
            byte as char
        })
        .collect()
}

pub fn pixelate_encode(pixel:&mut Vec<u8>,data:&str,channel:&mut u8) -> Result<Vec<u8>,&'static str>{
   LSB::encode(pixels,data,channel)
}
pub fn pixelate_decode(pixel:&mut Vec<u8>,channel:&mut u8) -> Result<Vec<u8>,&'static str>{
  LSB::decode(pixels,channel)
}

