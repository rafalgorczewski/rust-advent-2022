use super::utilities::load_file;
use std::collections::{HashMap, HashSet};

pub fn first() -> String {
  load_file("day3.txt")
    .into_iter()
    .map(|x| {
      let halves = x.split_at(x.len() / 2);
      let left_types = halves.0.as_bytes().iter().collect::<HashSet<_>>();
      let right_types = halves.1.as_bytes().iter().collect::<HashSet<_>>();
      left_types
        .into_iter()
        .filter(|x| right_types.contains(x))
        .map(|x| get_priority(*x))
        .sum::<i32>()
    })
    .sum::<i32>()
    .to_string()
}

pub fn second() -> String {
  load_file("day3.txt")
    .chunks(3)
    .map(|group| {
      let mut item_types: HashMap<u8, i32> = HashMap::new();
      group.iter().for_each(|knapsack| {
        knapsack
          .as_bytes()
          .iter()
          .collect::<HashSet<_>>()
          .into_iter()
          .for_each(|item| *item_types.entry(*item).or_default() += 1)
      });
      get_priority(item_types.into_iter().find(|x| (*x).1 == 3).unwrap().0)
    })
    .sum::<i32>()
    .to_string()
}

fn get_priority(ch: u8) -> i32 {
  let ch = ch as i32;
  match ch {
    65..=90 => ch - 38,
    97..=122 => ch - 96,
    _ => 0,
  }
}
