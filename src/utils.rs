pub fn vec_compare(va: &[u32], vb: &[u32]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter() // iterate
       .zip(vb) // zipa
       .all(|(a,b)| a==b) // compara
}
