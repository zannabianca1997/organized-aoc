#![feature(iter_next_chunk)]

use std::mem;

pub fn parse(input: &str) -> [[bool; 102]; 102] {
    Some([false; 102])
        .into_iter()
        .chain(input.trim().lines().map(|l| {
            Some(false)
                .into_iter()
                .chain(l.bytes().map(|b| match b {
                    b'.' => false,
                    b'#' => true,
                    _ => panic!(),
                }))
                .chain(Some(false))
                .next_chunk()
                .unwrap()
        }))
        .chain(Some([false; 102]))
        .next_chunk()
        .unwrap()
}

fn conway(current: &mut [[bool; 102]; 102], next: &mut [[bool; 102]; 102]) {
    for i in 1..101 {
        for j in 1..101 {
            let neighbours = [
                (i - 1, j - 1),
                (i, j - 1),
                (i + 1, j - 1),
                (i - 1, j),
                (i + 1, j),
                (i - 1, j + 1),
                (i, j + 1),
                (i + 1, j + 1),
            ]
            .into_iter()
            .filter(|(i, j)| current[*i][*j])
            .count();
            next[i][j] = if current[i][j] {
                neighbours == 2 || neighbours == 3
            } else {
                neighbours == 3
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut current = parse(input);
    let mut next = [[false; 102]; 102];
    for _ in 0..100 {
        conway(&mut current, &mut next);
        mem::swap(&mut current, &mut next);
    }
    current.into_iter().flatten().filter(|x| *x).count()
}

pub fn part2(input: &str) -> usize {
    let mut current = parse(input);
    let mut next = [[false; 102]; 102];
    for _ in 0..100 {
        current[1][1] = true;
        current[1][100] = true;
        current[100][1] = true;
        current[100][100] = true;
        conway(&mut current, &mut next);
        mem::swap(&mut current, &mut next);
    }

    current[1][1] = true;
    current[1][100] = true;
    current[100][1] = true;
    current[100][100] = true;
    current.into_iter().flatten().filter(|x| *x).count()
}
