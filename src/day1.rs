#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32>
{
    input
        .lines()
        .map(|l| { l.parse().unwrap() })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let first = input.iter();
    let second = input.iter().skip(1);
  
    first.zip(second).map(|(f, s)| s - f).map(|v| if v > 0 { 1 } else { 0 }).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let first = input.iter();
    let second = input.iter().skip(3);

    first.zip(second).map(|(f, s)| s - f).map(|v| if v > 0 { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
  
}
