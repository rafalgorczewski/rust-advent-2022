use super::utilities::{load_file, Direction, Position};
use std::collections::HashSet;

struct Rope {
  head: Position,
  prev_head: Position,
  pub tail: Position,
}

impl Rope {
  pub fn new() -> Self {
    Rope {
      head: Position::ZERO,
      prev_head: Position::ZERO,
      tail: Position::ZERO,
    }
  }

  pub fn move_towards(&mut self, direction: Direction) {
    self.prev_head = self.head;
    self.head += direction.to_position();
    if self.tail_too_far() {
      self.tail_follow();
    }
  }

  fn tail_too_far(&self) -> bool {
    let diff = self.diff().abs();
    diff.x > 1 || diff.y > 1
  }

  fn diff(&self) -> Position {
    self.head - self.tail
  }

  fn tail_follow(&mut self) {
    self.tail = self.prev_head;
  }
}

pub fn first() -> String {
  let mut rope = Rope::new();
  let mut tail_positions = HashSet::new();
  load_file("inputs/day9.txt").into_iter().for_each(|line| {
    let command = line.split_whitespace().collect::<Vec<_>>();
    for _ in 0..command[1].parse::<i32>().unwrap() {
      match command[0] {
        "U" => rope.move_towards(Direction::Up),
        "D" => rope.move_towards(Direction::Down),
        "L" => rope.move_towards(Direction::Left),
        "R" => rope.move_towards(Direction::Right),
        _ => (),
      }
      tail_positions.insert(rope.tail);
    }
  });
  tail_positions.len().to_string()
}

pub fn second() -> String {
  load_file("inputs/day9.txt");
  ""
}
