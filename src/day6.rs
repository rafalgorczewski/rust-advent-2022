use super::utilities::load_file_string;

use std::collections::HashSet;

pub fn first() -> String {
  (load_file_string("inputs/day6.txt")
    .as_bytes()
    .windows(4)
    .position(|x| {
      let mut letters_set = HashSet::new();
      x.iter().all(|ch| letters_set.insert(ch))
    })
    .unwrap()
    + 4)
    .to_string()
}

pub fn second() -> String {
  (load_file_string("inputs/day6.txt")
    .as_bytes()
    .windows(14)
    .position(|x| {
      let mut letters_set = HashSet::new();
      x.iter().all(|ch| letters_set.insert(ch))
    })
    .unwrap()
    + 14)
    .to_string()
}
