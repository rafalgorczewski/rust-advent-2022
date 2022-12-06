use super::utilities::load_file_split;

pub fn first() -> String {
  let input = load_file_split("inputs/day5.txt", "");
  let (stacks_desc, orders) = (&input[0], &input[1]);

  let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stacks_desc[0].len()];
  for i in 0..stacks_desc[0].len() {
    for line in stacks_desc.iter().rev() {
      stacks[i].push(line.as_bytes()[i]);
    }
  }
  stacks.retain(|x| x[0] != b' ');
  for stack in &mut stacks {
    stack.retain(|ch| ch.is_ascii_alphabetic());
  }

  orders.iter().for_each(|order| {
    let order_numbers = order
      .as_bytes()
      .iter()
      .filter(|ch| ch.is_ascii_digit() || ch.is_ascii_whitespace())
      .map(|ch| *ch as char)
      .collect::<String>()
      .split_whitespace()
      .map(|x| x.parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    let (quantity, source, destination) = (
      order_numbers[0],
      (order_numbers[1] - 1),
      (order_numbers[2] - 1),
    );
    for _ in 0..quantity {
      let package = stacks[source].pop().unwrap();
      stacks[destination].push(package);
    }
  });

  stacks
    .into_iter()
    .filter(|stack| !stack.is_empty())
    .map(|stack| *stack.last().unwrap() as char)
    .collect::<String>()
}

pub fn second() -> String {
  let input = load_file_split("inputs/day5.txt", "");
  let (stacks_desc, orders) = (&input[0], &input[1]);

  let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stacks_desc[0].len()];
  for i in 0..stacks_desc[0].len() {
    for line in stacks_desc.iter().rev() {
      stacks[i].push(line.as_bytes()[i]);
    }
  }
  stacks.retain(|x| x[0] != b' ');
  for stack in &mut stacks {
    stack.retain(|ch| ch.is_ascii_alphabetic());
  }

  orders.iter().for_each(|order| {
    let order_numbers = order
      .as_bytes()
      .iter()
      .filter(|ch| ch.is_ascii_digit() || ch.is_ascii_whitespace())
      .map(|ch| *ch as char)
      .collect::<String>()
      .split_whitespace()
      .map(|x| x.parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    let (quantity, source, destination) = (
      order_numbers[0],
      (order_numbers[1] - 1),
      (order_numbers[2] - 1),
    );
    let mut stack_to_reverse = Vec::new();
    for _ in 0..quantity {
      stack_to_reverse.push(stacks[source].pop().unwrap());
    }
    for package in stack_to_reverse.into_iter().rev() {
      stacks[destination].push(package);
    }
  });

  stacks
    .into_iter()
    .filter(|stack| !stack.is_empty())
    .map(|stack| *stack.last().unwrap() as char)
    .collect::<String>()
}
