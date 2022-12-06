use std::fs::read_to_string;

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
