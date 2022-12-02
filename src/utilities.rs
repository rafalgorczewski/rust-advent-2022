use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

pub fn load_file<T>(name: &str) -> Vec<String>
where
  T: FromStr,
  T::Err: Debug,
  Vec<String>: FromIterator<T>,
{
  read_to_string(name)
    .expect("File not found!")
    .lines()
    .map(|x| x.parse::<T>().unwrap())
    .collect()
}

pub fn load_file_split<T>(name: &str, delimiter: &str) -> Vec<Vec<String>>
where
  T: FromStr,
  T::Err: Debug,
  Vec<String>: FromIterator<T>,
{
  read_to_string(name)
    .expect("File not found!")
    .lines()
    .map(|x| x.parse::<T>().unwrap())
    .collect::<Vec<String>>()
    .split(|x| x == delimiter)
    .map(|x| x.to_owned())
    .collect()
}
