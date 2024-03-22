//first the json body with  image
//get pixels ✓
//block swap it ✓

pub mod block {
    pub fn rotate(vec: &mut Vec<u8>, d: u32) -> Vec<u8> {
        let n = vec.len() as u32;
        let d = d % n;
        let mut vec_clone = vec.clone();
        reverse(&mut vec_clone, 0, d - 1);
        reverse(&mut vec_clone, d, n - 1);
        reverse(&mut vec_clone, 0, n - 1);
        *vec = vec_clone;
        println!("rotate");
        vec.to_vec()
    }

    fn reverse(vec: &mut Vec<u8>, start: u32, end: u32) -> Vec<u8> {
        let mut i = start;
        let mut j = end;
        while i < j {
            vec.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
        println!("reversed");
        vec.clone()
    }
}
