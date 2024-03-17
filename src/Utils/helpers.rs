


//first the json body with  image
//get pixels ✓
//block swap it ✓
//return to UI



pub mod Block {
    pub async fn rotate(vec: &mut Vec<u8>, d: usize) -> &Vec<u8> {
        let n = vec.len();
        let d = d % n;
        let mut vec_clone = vec.clone();
        reverse(&mut vec_clone, 0, d - 1).await;
        reverse(&mut vec_clone, d, n - 1).await;
        reverse(&mut vec_clone, 0, n - 1).await;
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

