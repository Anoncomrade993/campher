/************************************************
Copyright (c) 2024 Dessxvii(Destiny Ifeanyi)
All rights reserved.

 This code is the intellectual property of Dessxvii(Destiny Ifeanyi). You are granted a non-exclusive, non-transferable, revocable license to use this code for personal or educational purposes only. Any unauthorized use, reproduction, or distribution of this code without the express permission of Dessxvii (Destiny Ifeanyi) is strictly prohibited.

This code is provided "as is" without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and non-infringement. In no event shall Dessxvii (Destiny Ifeanyi) be liable for any claim, damages, or other liability arising from the use of this code.

If you wish to use this code for commercial purposes or require a different licensing arrangement, please contact destinyifeanyi001@gmail.com.

Unauthorized copying or distribution of this code will be considered a violation of intellectual property rights and may result in legal action.
************************************************/

//Applying block swap algorithm
//https://en.wikipedia.org/wiki/Block_swap_algorithms

pub mod block_algorithm{
  
  pub fn rotate_vec(pixels:&mut Vec<u8>,d:usize)-> Vec<u8>{
    let n = pixels.len();
    let d = d % n;
    
    reverse_vec(&mut pixels,0,d-1);
    reverse_vec(&mut pixels,d,n-1);
    reverse_vec(&mut pixels,0,n-1);
   pixels
  }
  
  fn reverse_vec(pixels:&mut Vec<u8>,start:usize,end:usize)->(){
     let mid = (start + end ) /2;
     for i start..=mid{
       let j = (start + end - 1);
       pixels.swap(i,j);
     }
  }
}