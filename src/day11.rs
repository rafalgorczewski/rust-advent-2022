use super::utilities::load_file_split;

enum Op {
  AddOld,
  MulOld,
  Add(i32),
  Mul(i32),
}

impl Default for Op {
  fn default() -> Self {
    Op::Add(0)
  }
}

#[derive(Default)]
struct Monkey {
  items: Vec<u128>,
  op: Op,
  divisible_by: i32,
  if_true: i32,
  if_false: i32,
  pub activity: usize,
}

impl Monkey {
  pub fn inspect_items(&mut self, with_relief: bool) -> Vec<(i32, u128)> {
    let mut to_throw = Vec::new();
    for item in self.items.iter() {
      let mut item = match self.op {
        Op::AddOld => item + item,
        Op::MulOld => item * item,
        Op::Add(val) => item + val as u128,
        Op::Mul(val) => item * val as u128,
      };
      if with_relief {
        item = item.div_floor(3);
      }
      if (item % self.divisible_by as u128) == 0 {
        to_throw.push((self.if_true, item));
      } else {
        to_throw.push((self.if_false, item));
      }
      self.activity += 1;
    }
    self.items.clear();

    to_throw
  }

  pub fn receive_item(&mut self, item: u128) {
    self.items.push(item);
  }
}

fn parse_items(mut items_line: String) -> Vec<u128> {
  items_line.remove_matches("  Starting items: ");
  items_line.retain(|ch| ch != ',');
  items_line
    .split_whitespace()
    .map(|x| x.parse::<u128>().unwrap())
    .collect()
}

fn parse_op(mut op_line: String) -> Op {
  op_line.remove_matches("  Operation: new = old ");
  if op_line == "+ old" {
    return Op::AddOld;
  } else if op_line == "* old" {
    return Op::MulOld;
  }

  let op_desc = op_line.split_whitespace().collect::<Vec<_>>();
  let (op, val) = (op_desc[0], op_desc[1].parse().unwrap());
  match op {
    "+" => Op::Add(val),
    "*" => Op::Mul(val),
    _ => Op::Add(0),
  }
}

fn parse_divisible_by(mut divisible_line: String) -> i32 {
  divisible_line.remove_matches("  Test: divisible by ");
  divisible_line.parse().unwrap()
}

fn parse_if_true(mut divisible_line: String) -> i32 {
  divisible_line.remove_matches("    If true: throw to monkey ");
  divisible_line.parse().unwrap()
}

fn parse_if_false(mut divisible_line: String) -> i32 {
  divisible_line.remove_matches("    If false: throw to monkey ");
  divisible_line.parse().unwrap()
}

pub fn first() -> String {
  let mut monkeys = load_file_split("inputs/day11.txt", "")
    .into_iter()
    .map(|monkey_desc| {
      let mut monkey = Monkey::default();

      monkey.items = parse_items(monkey_desc[1].clone());
      monkey.op = parse_op(monkey_desc[2].clone());
      monkey.divisible_by = parse_divisible_by(monkey_desc[3].clone());
      monkey.if_true = parse_if_true(monkey_desc[4].clone());
      monkey.if_false = parse_if_false(monkey_desc[5].clone());

      monkey
    })
    .collect::<Vec<_>>();

  for _ in 0..20 {
    for i in 0..monkeys.len() {
      let items = monkeys[i].inspect_items(true);
      for item in items {
        monkeys[item.0 as usize].receive_item(item.1);
      }
    }
  }
  monkeys.sort_by(|lhs, rhs| rhs.activity.cmp(&lhs.activity));
  (monkeys[0].activity * monkeys[1].activity).to_string()
}

pub fn second() -> String {
  let mut monkeys = load_file_split("inputs/day11.txt", "")
    .into_iter()
    .map(|monkey_desc| {
      let mut monkey = Monkey::default();

      monkey.items = parse_items(monkey_desc[1].clone());
      monkey.op = parse_op(monkey_desc[2].clone());
      monkey.divisible_by = parse_divisible_by(monkey_desc[3].clone());
      monkey.if_true = parse_if_true(monkey_desc[4].clone());
      monkey.if_false = parse_if_false(monkey_desc[5].clone());

      monkey
    })
    .collect::<Vec<_>>();

  for _ in 0..20 {
    for i in 0..monkeys.len() {
      let items = monkeys[i].inspect_items(false);
      for item in items {
        monkeys[item.0 as usize].receive_item(item.1);
      }
    }
  }
  monkeys.sort_by(|lhs, rhs| rhs.activity.cmp(&lhs.activity));
  (monkeys[0].activity * monkeys[1].activity).to_string()
}
