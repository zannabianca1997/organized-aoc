#![feature(iter_array_chunks)]
use std::collections::BTreeMap;

use grid::Grid;

pub fn parse(input: &str) -> Grid<usize> {
    let input: Vec<(&str, &str, usize)> = input
        .split_ascii_whitespace()
        .array_chunks()
        .map(|[a, _, b, _, c]| (a, b, c.parse().unwrap()))
        .collect();
    let names: BTreeMap<&str, usize> = {
        let mut counter = 0usize;
        let mut found = BTreeMap::new();
        for (a, b, _) in input.iter() {
            for n in [a, b] {
                if !found.contains_key(n) {
                    found.insert(*n, counter);
                    counter += 1;
                }
            }
        }
        found
    };
    let mut grid = Grid::new(names.len(), names.len());
    for (a, b, d) in input {
        let a = names[a];
        let b = names[b];
        grid[(a, b)] = d;
        grid[(b, a)] = d;
    }
    grid
}

pub fn shortest(
    distances: &Grid<usize>,
    current: usize,
    mut unvisited: Vec<usize>,
    budget: usize,
) -> usize {
    if unvisited.is_empty() {
        return 0; // no more city to visit
    }
    let mut min = budget;
    unvisited.sort_unstable_by_key(|c| distances[(current, *c)]);
    for (idx, city) in unvisited.iter().enumerate() {
        let d = distances[(current, *city)];
        if d <= min {
            let mut new_unvisited = Vec::with_capacity(unvisited.len() - 1);
            new_unvisited.extend_from_slice(&unvisited[..idx]);
            new_unvisited.extend_from_slice(&unvisited[idx + 1..]);
            let min_by_city = d + shortest(distances, *city, new_unvisited, min - d);
            min = min.min(min_by_city);
        }
    }
    min
}

pub fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut min = usize::MAX;
    for start in 0..input.rows() {
        let min_from_city = shortest(
            &input,
            start,
            (0..input.rows())
                .filter(|x| *x != start)
                .collect::<Vec<_>>(),
            min,
        );
        min = min.min(min_from_city);
    }
    min
}

const CACHE_THRESHOLD: usize = 2;

pub fn longest(
    distances: &Grid<usize>,
    current: usize,
    unvisited: Vec<usize>,
    cache: &mut Vec<BTreeMap<Vec<usize>, usize>>,
) -> usize {
    if unvisited.is_empty() {
        return 0; // no more city to visit
    }
    if unvisited.len() > CACHE_THRESHOLD {
        if let Some(cached) = cache[current].get(&unvisited) {
            return *cached;
        }
    }
    let mut max = usize::MIN;

    for (idx, city) in unvisited.iter().enumerate() {
        let d = distances[(current, *city)];

        let mut new_unvisited = Vec::with_capacity(unvisited.len() - 1);
        new_unvisited.extend_from_slice(&unvisited[..idx]);
        new_unvisited.extend_from_slice(&unvisited[idx + 1..]);
        let max_by_city = d + longest(distances, *city, new_unvisited, cache);
        max = max.max(max_by_city);
    }
    if unvisited.len() > CACHE_THRESHOLD {
        cache[current].insert(unvisited, max);
    }
    max
}

pub fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut max = usize::MIN;
    for start in 0..input.rows() {
        let max_from_city = longest(
            &input,
            start,
            (0..input.rows())
                .filter(|x| *x != start)
                .collect::<Vec<_>>(),
            &mut vec![BTreeMap::new(); input.rows()],
        );
        max = max.max(max_from_city);
    }
    max
}
