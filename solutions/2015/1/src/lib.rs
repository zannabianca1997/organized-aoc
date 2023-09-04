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

pub fn part2(input: &str) -> i64 {
    input
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .scan(0isize, |s, d| {
            *s += d;
            Some(*s)
        })
        .take_while(|h| *h >= 0)
        .count() as _
}
