#![feature(string_remove_matches)]
#![feature(int_roundings)]

mod utilities;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
  println!("First: {}", day11::first());
  println!("Second: {}", day11::second());
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
