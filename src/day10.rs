use super::utilities::load_file;

enum Instruction {
  AddX(i32),
  Noop,
}

struct Cpu {
  register: i32,
  cycle_number: i32,
  instructions: Vec<Instruction>,
  screen: Vec<Vec<u8>>,
}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      register: 1,
      cycle_number: 1,
      instructions: Vec::new(),
      screen: vec![vec![b' '; 40]; 6],
    }
  }

  pub fn addx(&mut self, val: i32) {
    self.instructions.push(Instruction::Noop);
    self.instructions.push(Instruction::AddX(val));
  }

  pub fn noop(&mut self) {
    self.instructions.push(Instruction::Noop);
  }

  pub fn eval_next_cycles(&mut self, cycle: i32) -> i32 {
    while self.cycle_number < cycle {
      if let Instruction::AddX(val) = self.instructions[self.cycle_number as usize - 1] {
        self.register += val;
      }

      let sprite_x = self.register;
      let cursor_x = self.cycle_number % 40;
      let cursor_y = (self.cycle_number / 40) % 6;
      let has_to_draw =
        (sprite_x - 1 == cursor_x) || (sprite_x == cursor_x) || (sprite_x + 1 == cursor_x);
      if has_to_draw {
        self.screen[cursor_y as usize][cursor_x as usize] = b'#';
      }

      self.cycle_number += 1;
    }
    self.register * self.cycle_number
  }

  pub fn print_screen(&self) {
    for line in self.screen.iter() {
      println!("{}", String::from_utf8(line.clone()).unwrap())
    }
  }
}

pub fn first() -> String {
  let mut cpu = Cpu::new();
  load_file("inputs/day10.txt").into_iter().for_each(|line| {
    if line == "noop" {
      cpu.noop();
    } else {
      cpu.addx(
        line.split_whitespace().collect::<Vec<_>>()[1]
          .parse::<i32>()
          .unwrap(),
      );
    }
  });

  let mut strengths_sum = 0;
  for cycle in [20, 60, 100, 140, 180, 220] {
    let strength = cpu.eval_next_cycles(cycle);
    strengths_sum += strength;
  }

  strengths_sum.to_string()
}

pub fn second() {
  let mut cpu = Cpu::new();
  load_file("inputs/day10.txt").into_iter().for_each(|line| {
    if line == "noop" {
      cpu.noop();
    } else {
      cpu.addx(
        line.split_whitespace().collect::<Vec<_>>()[1]
          .parse::<i32>()
          .unwrap(),
      );
    }
  });

  for cycle in [40, 80, 120, 160, 200, 240] {
    let _ = cpu.eval_next_cycles(cycle);
  }
  cpu.print_screen();
}
