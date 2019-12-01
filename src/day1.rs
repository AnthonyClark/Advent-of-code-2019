#[aoc_generator(day1, part1, withGen)]
pub fn part1_weights(input: &str) -> Vec<i32> {
  input.lines().map(|l| {
    l.parse::<i32>().unwrap()
  }).collect()
}

#[aoc(day1, part1, withGen)]
pub fn part1_fuel(weights: &[i32]) -> i32 {
  weights.iter().map(|w| {
    (w / 3) - 2
  }).sum()
}

#[aoc(day1, part1, noGen)]
pub fn part1_shorter(input: &str) -> i32 {
  input.lines().map(|l| {
    (l.parse::<i32>().unwrap() / 3) - 2
  }).sum()
}

#[cfg(test)]
mod tests {

  #[test]
  fn rust_divison() {
    assert_eq!((3/2), 1);
  }
}
