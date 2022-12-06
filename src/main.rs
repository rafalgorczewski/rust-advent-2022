mod utilities;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
  println!("Day 5 (first): {}", day5::first());
  println!("Day 5 (second): {}", day5::second());
}

// use super::utilities::load_file;
//
// pub fn first() -> String {
//   load_file("inputs/day1.txt");
// }
//
// pub fn second() -> String {
//   load_file("inputs/day1.txt");
// }
