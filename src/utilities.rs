use std::fs::read_to_string;
use std::ops;

pub fn load_file_string(name: &str) -> String {
  read_to_string(name).expect("File not found!")
}

pub fn load_file(name: &str) -> Vec<String> {
  load_file_string(name)
    .lines()
    .map(|x| x.to_owned())
    .collect()
}

pub fn load_file_split(name: &str, delimiter: &str) -> Vec<Vec<String>> {
  load_file(name)
    .split(|x| x == delimiter)
    .map(|x| x.to_owned())
    .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

impl Position {
  pub const ZERO: Position = Position { x: 0, y: 0 };
  pub const ONE: Position = Position { x: 1, y: 1 };
  pub const MINUS_ONE: Position = Position { x: -1, y: -1 };
  pub const UP: Position = Position { x: 0, y: -1 };
  pub const DOWN: Position = Position { x: 0, y: 1 };
  pub const LEFT: Position = Position { x: -1, y: 0 };
  pub const RIGHT: Position = Position { x: 1, y: 0 };

  pub fn new(x: i32, y: i32) -> Position {
    Position { x, y }
  }

  pub fn abs(&self) -> Position {
    Position {
      x: self.x.abs(),
      y: self.y.abs(),
    }
  }
}

impl ops::Add for Position {
  type Output = Position;
  fn add(self, rhs: Self) -> Self::Output {
    Position {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl ops::AddAssign for Position {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl ops::Sub for Position {
  type Output = Position;
  fn sub(self, rhs: Self) -> Self::Output {
    Position {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl ops::SubAssign for Position {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub const fn to_position(&self) -> Position {
    match *self {
      Self::Up => Position::UP,
      Self::Down => Position::DOWN,
      Self::Left => Position::LEFT,
      Self::Right => Position::RIGHT,
    }
  }
}
