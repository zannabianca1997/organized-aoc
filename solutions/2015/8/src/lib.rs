#![feature(iter_advance_by)]

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut bytes = l[1..l.len() - 1].bytes();
            let mut diff = 2;
            while let Some(a) = bytes.next() {
                if a == b'\\' {
                    match bytes.next().unwrap() {
                        b'\\' | b'"' => diff += 1,
                        b'x' => {
                            bytes.advance_by(2).unwrap(); // consume the hexes
                            diff += 3;
                        }
                        _ => panic!(),
                    }
                }
            }
            diff
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| 2 + l.bytes().filter(|a| *a == b'\\' || *a == b'"').count())
        .sum()
}
