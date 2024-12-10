use std::{collections::HashSet, hash::BuildHasher};

use grid::Grid;

pub fn parse(input: &str) -> Grid<u8> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid = Grid::new(height, width, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch as u8 - b'0';
        }
    }
    grid
}

pub fn part1(input: &str) -> usize {
    let input = parse(input);

    let mut total_score = 0;

    for x in 0..input.shape().0 {
        for y in 0..input.shape().1 {
            if input[(x, y)] == 0 {
                // Is a trail head
                let mut tails = fnv::FnvHashSet::default();
                score(x, y, &input, 0, &mut tails);
                total_score += tails.len();
            }
        }
    }

    total_score
}

fn score<S: BuildHasher>(
    x: usize,
    y: usize,
    input: &Grid<u8>,
    height: u8,
    tails: &mut HashSet<(usize, usize), S>,
) {
    debug_assert_eq!(input[(x, y)], height);
    if height == 9 {
        tails.insert((x, y));
        return;
    }
    let next_height = height + 1;
    if input
        .get(x.wrapping_add(1), y)
        .is_some_and(|v| *v == next_height)
    {
        score(x.wrapping_add(1), y, input, next_height, tails)
    }
    if input
        .get(x.wrapping_sub(1), y)
        .is_some_and(|v| *v == next_height)
    {
        score(x.wrapping_sub(1), y, input, next_height, tails)
    }
    if input
        .get(x, y.wrapping_add(1))
        .is_some_and(|v| *v == next_height)
    {
        score(x, y.wrapping_add(1), input, next_height, tails)
    }
    if input
        .get(x, y.wrapping_sub(1))
        .is_some_and(|v| *v == next_height)
    {
        score(x, y.wrapping_sub(1), input, next_height, tails)
    }
}

fn score2(x: usize, y: usize, input: &Grid<u8>, height: u8) -> usize {
    debug_assert_eq!(input[(x, y)], height);
    if height == 9 {
        return 1;
    }
    let next_height = height + 1;
    let mut total = 0;
    if input
        .get(x.wrapping_add(1), y)
        .is_some_and(|v| *v == next_height)
    {
        total += score2(x.wrapping_add(1), y, input, next_height)
    }
    if input
        .get(x.wrapping_sub(1), y)
        .is_some_and(|v| *v == next_height)
    {
        total += score2(x.wrapping_sub(1), y, input, next_height)
    }
    if input
        .get(x, y.wrapping_add(1))
        .is_some_and(|v| *v == next_height)
    {
        total += score2(x, y.wrapping_add(1), input, next_height)
    }
    if input
        .get(x, y.wrapping_sub(1))
        .is_some_and(|v| *v == next_height)
    {
        total += score2(x, y.wrapping_sub(1), input, next_height)
    }

    total
}

pub fn part2(input: &str) -> usize {
    let input = parse(input);

    let mut total_score = 0;

    for x in 0..input.shape().0 {
        for y in 0..input.shape().1 {
            if input[(x, y)] == 0 {
                // Is a trail head
                total_score += score2(x, y, &input, 0);
            }
        }
    }

    total_score
}
