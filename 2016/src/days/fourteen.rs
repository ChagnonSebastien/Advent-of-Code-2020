use crate::utils::Part;
use std::collections::{BTreeMap, HashSet};
use md5;

pub fn execute(input: String, part: &Part) {
  let mut keys: HashSet<usize> = HashSet::new();
  let mut potentials: BTreeMap<usize, char> = BTreeMap::new();

  let mut i = 0;
  while keys.len() < 64 {
    let mut digest = md5::compute(format!("{}{}", input, i));
    let mut hash = format!("{:x}", digest);
    
    if *part == Part::PartTwo {
      for _ in 0..2016 {
        digest = md5::compute(format!("{}", hash));
        hash = format!("{:x}", digest);
      }
    }

    let chars = hash.chars().collect::<Vec<char>>();
    for j in 0..chars.len()-2 {
      if chars[j] == chars[j+1] && chars[j+1] == chars[j+2] {
        potentials.insert(i, chars[j]);
        break;
      }
    }
    for j in 0..chars.len()-4 {
      if chars[j] == chars[j+1] && chars[j+1] == chars[j+2] && chars[j+2] == chars[j+3] && chars[j+3] == chars[j+4] {
        let c = chars[j];
        for (key_index, key_char) in potentials.range(match i < 1000 { true => 0, false => i-1000 }..i) {
          if *key_char == c && keys.len() < 64 {
            keys.insert(*key_index);
          }
        }
      }
    }

    i += 1;
  }

  let mut last = 0;
  for key in &keys {
    if *key > last {
      last = *key;
    }
  }
  println!("Found the {}-th key. Last key index: {}", keys.len(), last);
}
