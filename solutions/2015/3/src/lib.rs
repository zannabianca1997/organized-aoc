#![feature(array_chunks)]
use std::collections::BTreeSet;

pub fn part1(input: &str) -> usize {
    let mut visited = BTreeSet::from([[0, 0]]);
    let mut x = 0isize;
    let mut y = 0isize;
    for cmd in input.bytes() {
        match cmd {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => panic!("Unknown direction {}", cmd as char),
        }
        visited.insert([x, y]);
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut visited = BTreeSet::from([[0, 0]]);
    let mut x = 0isize;
    let mut y = 0isize;
    let mut rx = 0isize;
    let mut ry = 0isize;
    let mut chunkes = input.as_bytes().array_chunks();
    for [cmd, rcmd] in &mut chunkes {
        match cmd {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => panic!("Unknown direction {}", *cmd as char),
        }
        visited.insert([x, y]);
        match rcmd {
            b'>' => rx += 1,
            b'<' => rx -= 1,
            b'^' => ry += 1,
            b'v' => ry -= 1,
            _ => panic!("Unknown direction {}", *rcmd as char),
        }
        visited.insert([rx, ry]);
    }
    if let [cmd] = chunkes.remainder() {
        match cmd {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y += 1,
            b'v' => y -= 1,
            _ => panic!("Unknown direction {}", *cmd as char),
        }
        visited.insert([x, y]);
    }
    visited.len()
}
