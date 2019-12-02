#[aoc_generator(day2)]
pub fn part1_gen(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|i| i.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn part1_sol(input: &[i32]) -> i32 {
  let mut int_codes: Vec<i32> = input.to_vec();

  int_codes[1] = 12;
  int_codes[2] = 2;
  return intcode_runner(&mut int_codes);
}

#[aoc(day2, part2)]
pub fn part2_sol(input: &[i32]) -> i32 {
  let mut int_codes: Vec<i32>;
  let mut i: i32 = 0;
  let mut j: i32;
  'outer: loop {
    j = 0;

    'inner: loop {
      int_codes = input.to_vec(); // Fresh input
      int_codes[1] = i;
      int_codes[2] = j;

      let res: i32 = intcode_runner(&mut int_codes);
      if res == 19690720 {
        break 'outer;
      } else if j > 99 {
        break 'inner;
      } else {
        j = j + 1
      }
    }
    if i > 99 {
      break;
    } else {
      i = i + 1;
    }
  }

  return 100 * i + j;
}

pub fn intcode_runner(int_codes: &mut Vec<i32>) -> i32 {
  let mut pc: usize = 0;

  while pc < int_codes.len() {
    let op = int_codes[pc];
    match op {
      99 => break,
      1 | 2 => {
        if pc + 3 > int_codes.len() - 1 {
          println!("pc out of bounds");
          break;
        }
        let i = int_codes[pc + 1] as usize;
        let j = int_codes[pc + 2] as usize;
        let k = int_codes[pc + 3] as usize;
        int_codes[k] = match op {
          1 => int_codes[i] + int_codes[j],
          2 => int_codes[i] * int_codes[j],
          _ => unreachable!(),
        };
        pc = pc + 4;
      }
      _ => {
        println!("Unrecognized opcode: {}", op);
        break;
      }
    }
  }
  return int_codes[0];
}
