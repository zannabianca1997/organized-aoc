//! Aoc year 2015 day 1
pub fn part1(input: &str) -> i64 {
    input
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}
