/************************************************
Copyright (c) 2024 Dessxvii(Destiny Ifeanyi)
All rights reserved.

 This code is the intellectual property of Dessxvii(Destiny Ifeanyi). You are granted a non-exclusive, non-transferable, revocable license to use this code for personal or educational purposes only. Any unauthorized use, reproduction, or distribution of this code without the express permission of Dessxvii (Destiny Ifeanyi) is strictly prohibited.

This code is provided "as is" without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and non-infringement. In no event shall Dessxvii (Destiny Ifeanyi) be liable for any claim, damages, or other liability arising from the use of this code.

If you wish to use this code for commercial purposes or require a different licensing arrangement, please contact destinyifeanyi001@gmail.com.

Unauthorized copying or distribution of this code will be considered a violation of intellectual property rights and may result in legal action.
************************************************/
use chrono::Utc
pub struct Data{
  pixels :mut Vec<u8>,
  pix_id :String,
  key :String
  created_at :String
}

impl  Data{
  pub fn new(pixels: mut Vec<u8>,key:&str,pix_id:&str) -> Self{
    Self { 
    pixels
    ,key:key.to_string()
    ,pix_id:pix_id.to_string(),
    created_at: Utc::now().to_rfc339()
    }
  }
}