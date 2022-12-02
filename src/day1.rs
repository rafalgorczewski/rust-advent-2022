use super::utilities::load_file_split;

pub fn first() -> String {
  load_file_split("day1.txt", "")
    .into_iter()
    .map(|chunk| chunk.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
    .max()
    .unwrap()
    .to_string()
}

pub fn second() -> String {
  let mut sums = load_file_split("day1.txt", "")
    .into_iter()
    .map(|chunk| chunk.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
    .collect::<Vec<i32>>();
  sums.sort_by(|lhs, rhs| rhs.cmp(lhs));
  sums.truncate(3);
  sums.into_iter().sum::<i32>().to_string()
}
