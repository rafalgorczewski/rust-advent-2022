use super::utilities::load_file;

fn is_visible((x, y): (usize, usize), forest_board: &Vec<Vec<u8>>) -> bool {
  if x == 0 || y == 0 || x + 1 == forest_board[0].len() || y + 1 == forest_board.len() {
    return true;
  }

  let height = forest_board[y][x];
  let mut max_height = 0;

  for x_left in (0..x).rev() {
    if forest_board[y][x_left] > max_height {
      max_height = forest_board[y][x_left];
    }
  }
  if height > max_height {
    return true;
  }
  max_height = 0;

  for x_right in (x + 1)..forest_board[0].len() {
    if forest_board[y][x_right] > max_height {
      max_height = forest_board[y][x_right];
    }
  }
  if height > max_height {
    return true;
  }
  max_height = 0;

  for y_up in (0..y).rev() {
    if forest_board[y_up][x] > max_height {
      max_height = forest_board[y_up][x];
    }
  }
  if height > max_height {
    return true;
  }
  max_height = 0;

  for y_down in (y + 1)..forest_board.len() {
    if forest_board[y_down][x] > max_height {
      max_height = forest_board[y_down][x];
    }
  }
  if height > max_height {
    return true;
  }

  false
}

fn scenic_score((x, y): (usize, usize), forest_board: &Vec<Vec<u8>>) -> i32 {
  let height = forest_board[y][x];
  let mut scenic_score = 0;

  if x == 0 || y == 0 || x + 1 == forest_board[0].len() || y + 1 == forest_board.len() {
    return 0;
  }

  for x_left in (0..x).rev() {
    if forest_board[y][x_left] >= height || x_left == 0 {
      scenic_score += x - x_left;
      break;
    }
  }

  for x_right in (x + 1)..forest_board[0].len() {
    if forest_board[y][x_right] >= height || x_right + 1 == forest_board[0].len() {
      scenic_score *= x_right - x;
      break;
    }
  }

  for y_up in (0..y).rev() {
    if forest_board[y_up][x] >= height || y_up == 0 {
      scenic_score *= y - y_up;
      break;
    }
  }

  for y_down in (y + 1)..forest_board.len() {
    if forest_board[y_down][x] >= height || y_down + 1 == forest_board.len() {
      scenic_score *= y_down - y;
      break;
    }
  }

  scenic_score as i32
}

pub fn first() -> String {
  let forest_board = load_file("inputs/day8.txt")
    .into_iter()
    .map(|x| x.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let mut visible_count = 0;
  for y in 0..forest_board.len() {
    for x in 0..forest_board[0].len() {
      if is_visible((x, y), &forest_board) {
        visible_count += 1;
      }
    }
  }
  visible_count.to_string()
}

pub fn second() -> String {
  let forest_board = load_file("inputs/day8.txt")
    .into_iter()
    .map(|x| x.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let mut max_score = 0;
  for y in 0..forest_board.len() {
    for x in 0..forest_board[0].len() {
      let score = scenic_score((x, y), &forest_board);
      if score > max_score {
        max_score = score;
      }
    }
  }
  max_score.to_string()
}
