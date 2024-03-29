#[aoc_generator(day1, part1, withGen)]
pub fn part1_weights(input: &str) -> Vec<i32> {
  input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1, withGen)]
pub fn part1_fuel(weights: &[i32]) -> i32 {
  weights.iter().map(|w| fuel_for_weight(*w)).sum()
}

#[aoc(day1, part1, noGen)]
pub fn part1_shorter(input: &str) -> i32 {
  input
    .lines()
    .map(|l| l.parse().unwrap())
    .map(fuel_for_weight)
    .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
  input
    .lines()
    .map(|l| l.parse().unwrap())
    .map(total_fuel_for_module)
    .sum()
}

fn fuel_for_weight(input: i32) -> i32 {
  return std::cmp::max(input / 3 - 2, 0);
}

fn total_fuel_for_module(input: i32) -> i32 {
  let w1 = fuel_for_weight(input);
  let mut w2: i32 = 0;
  let mut w_tmp: i32 = fuel_for_weight(w1);

  while w_tmp > 0 {
    w2 = w2 + w_tmp;
    w_tmp = fuel_for_weight(w_tmp);
  }
  return w1 + w2;
}

#[cfg(test)]
mod tests {
  use crate::day1::fuel_for_weight;

  #[test]
  fn rust_divison() {
    assert_eq!((3 / 2), 1);
  }

  #[test]
  fn test_fueld_for_weight() {
    assert_eq!(fuel_for_weight(12), 2);
    assert_eq!(fuel_for_weight(13), 2);
    assert_eq!(fuel_for_weight(15), 3);
  }
}
