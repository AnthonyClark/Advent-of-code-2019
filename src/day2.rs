#[aoc_generator(day2, part1)]
pub fn part1_gen(input: &str) -> Vec<i32> {
  input.split(",").map(|i| {
    i.parse::<i32>().unwrap()
  }).collect()
}

#[aoc(day2, part1)]
pub fn part1_sol(input: &[i32]) -> i32 {
  let mut int_codes: Vec<i32> = input.to_vec();

  int_codes[1] = 12;
  int_codes[2] = 2;

  let mut pc: usize = 0;

  while pc < int_codes.len() {
    match int_codes[pc] {
      99 => break,
      1 =>  {
        let i = int_codes[pc + 1] as usize;
        let j = int_codes[pc + 2] as usize;
        let k = int_codes[pc + 3] as usize;
        int_codes[k] = int_codes[i] + int_codes[j];
      },
      2 =>  {
        let i = int_codes[pc + 1] as usize;
        let j = int_codes[pc + 2] as usize;
        let k = int_codes[pc + 3] as usize;
        int_codes[k] = int_codes[i] * int_codes[j];
      },
      _ => ()
    }
    pc = pc + 4;
  }
  return int_codes[0];
}