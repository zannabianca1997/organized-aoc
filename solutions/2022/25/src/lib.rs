fn parse_snafu(val: &str) -> isize {
    val.chars()
        .rev()
        .enumerate()
        .map(|(pow, ch)| {
            let v = match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                ch => panic!("{ch:?} is not a valid SNAFU digit"),
            };
            let exp = 5isize
                .checked_pow(pow as u32)
                .expect("SNAFU number too big");
            v * exp
        })
        .sum()
}
fn format_snafu(n: isize) -> String {
    let (last_digit, borrow) = match n.rem_euclid(5) {
        0 => ('0', 0),
        1 => ('1', 0),
        2 => ('2', 0),
        3 => ('=', 1),
        4 => ('-', 1),
        _ => unreachable!(),
    };
    let rest = n.div_euclid(5) + borrow;
    let mut pre = if rest > 0 {
        format_snafu(rest)
    } else {
        String::new()
    };
    pre.push(last_digit);
    pre
}

pub fn part1(input: &str) -> String {
    format_snafu(
        input
            .trim()
            .lines()
            .map(|line| parse_snafu(line.trim()))
            .sum(),
    )
}
