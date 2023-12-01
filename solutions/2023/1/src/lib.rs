pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let l = l.trim_matches(|c: char| !c.is_ascii_digit());
            (l.as_bytes()[0] - b'0') as u32 * 10 + (l.as_bytes()[l.len() - 1] - b'0') as u32
        })
        .sum()
}

fn start_with_digit(s: &str) -> Option<u32> {
    match s.as_bytes() {
        [b'0'..=b'9', ..] => Some((s.as_bytes().first().unwrap() - b'0') as _),
        [b'o', b'n', b'e', ..] => Some(1),
        [b't', b'w', b'o', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..] => Some(5),
        [b's', b'i', b'x', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..] => Some(9),
        [b'z', b'e', b'r', b'o', ..] => Some(0),
        _ => None,
    }
}
fn end_with_digit(s: &str) -> Option<u32> {
    match s.as_bytes() {
        [.., b'0'..=b'9'] => Some((s.as_bytes().last().unwrap() - b'0') as _),
        [.., b'o', b'n', b'e'] => Some(1),
        [.., b't', b'w', b'o'] => Some(2),
        [.., b't', b'h', b'r', b'e', b'e'] => Some(3),
        [.., b'f', b'o', b'u', b'r'] => Some(4),
        [.., b'f', b'i', b'v', b'e'] => Some(5),
        [.., b's', b'i', b'x'] => Some(6),
        [.., b's', b'e', b'v', b'e', b'n'] => Some(7),
        [.., b'e', b'i', b'g', b'h', b't'] => Some(8),
        [.., b'n', b'i', b'n', b'e'] => Some(9),
        [.., b'z', b'e', b'r', b'o'] => Some(0),
        _ => None,
    }
}
fn last_digit(mut s: &str) -> u32 {
    loop {
        if let Some(digit) = end_with_digit(s) {
            return digit;
        }
        s = &s[..s.len() - 1]
    }
}
fn first_digit(mut s: &str) -> u32 {
    loop {
        if let Some(digit) = start_with_digit(s) {
            return digit;
        }
        s = &s[1..]
    }
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| first_digit(l) * 10 + last_digit(l))
        .sum()
}
