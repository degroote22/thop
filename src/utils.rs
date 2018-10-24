use std::collections::HashMap;

pub fn vec_compare(va: &[u32], vb: &[u32]) -> bool {
  (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter() // iterate
       .zip(vb) // zipa
       .all(|(a,b)| a==b) // compara
}

pub fn _hash_compare(ha: &HashMap<u32, bool>, hb: &HashMap<u32, bool>) -> bool {
  let mut va = Vec::new();
  for (key, _value) in ha.iter() {
    va.push(*key);
  }
  va.sort();

  let mut vb = Vec::new();
  for (key, _value) in hb.iter() {
    vb.push(*key);
  }
  vb.sort();

  return vec_compare(&va, &vb);
}
