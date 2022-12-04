use super::utilities::load_file;

pub fn first() -> String {
  load_file("inputs/day4.txt")
    .into_iter()
    .filter(|pair| {
      let numbers = pair
        .split(&['-', ','][..])
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
      (numbers[0] <= numbers[2] && numbers[1] >= numbers[3])
        || (numbers[0] >= numbers[2] && numbers[1] <= numbers[3])
    })
    .count()
    .to_string()
}

pub fn second() -> String {
  load_file("inputs/day4.txt")
    .into_iter()
    .filter(|pair| {
      let mut numbers = pair
        .split(&['-', ','][..])
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
      (numbers[0] <= numbers[2] && numbers[1] >= numbers[3])
        || (numbers[0] >= numbers[2] && numbers[1] <= numbers[3])
        || (numbers[0] >= numbers[2] && numbers[0] <= numbers[3])
        || (numbers[1] >= numbers[2] && numbers[1] <= numbers[3])
    })
    .count()
    .to_string()
}
