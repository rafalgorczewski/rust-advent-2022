use super::utilities::load_file;
use std::collections::HashMap;

pub fn first() -> String {
  let wins_with = HashMap::from([('X', 'C'), ('Y', 'A'), ('Z', 'B')]);
  let map_to_opponent = HashMap::from([('X', 'A'), ('Y', 'B'), ('Z', 'C')]);
  let scores = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

  load_file("day2.txt")
    .into_iter()
    .map(|x| {
      let lhs = x.as_bytes()[0] as char;
      let rhs = x.as_bytes()[2] as char;
      scores[&rhs]
        + if lhs == wins_with[&rhs] {
          6
        } else if map_to_opponent[&rhs] == lhs {
          3
        } else {
          0
        }
    })
    .sum::<i32>()
    .to_string()
}

pub fn second() -> String {
  let wins_with = HashMap::from([('A', 'C'), ('B', 'A'), ('C', 'B')]);
  let loses_with = HashMap::from([('A', 'B'), ('B', 'C'), ('C', 'A')]);
  let sign_scores = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
  let condition_scores = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);

  load_file("day2.txt")
    .into_iter()
    .map(|x| {
      let lhs = x.as_bytes()[0] as char;
      let rhs = x.as_bytes()[2] as char;
      condition_scores[&rhs]
        + match rhs {
          'X' => sign_scores[&wins_with[&lhs]],
          'Y' => sign_scores[&lhs],
          'Z' => sign_scores[&loses_with[&lhs]],
          _ => 0,
        }
    })
    .sum::<i32>()
    .to_string()
}
