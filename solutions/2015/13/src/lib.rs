#![feature(iter_next_chunk)]

use std::collections::BTreeMap;

use grid::Grid;
use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = (&str, &str, isize)> {
    input.trim().lines().map(|l| {
        // Alice would lose 57 happiness units by sitting next to Bob.
        let [a, _, sign, n, _, _, _, _, _, _, b] = l
            .trim()
            .strip_suffix('.')
            .unwrap()
            .splitn(11, ' ')
            .next_chunk()
            .unwrap();
        let n = match sign {
            "lose" => -1,
            "gain" => 1,
            _ => panic!(),
        } * n.parse::<isize>().unwrap();
        (a, b, n)
    })
}

fn graph<'i>(instructions: impl Iterator<Item = (&'i str, &'i str, isize)>) -> Grid<isize> {
    // count the number of partecipants
    let instructions: Vec<_> = instructions.collect();
    let numbers: BTreeMap<_, _> = instructions
        .iter()
        .flat_map(|(a, b, _)| [*a, *b])
        .unique()
        .enumerate()
        .map(|(n, a)| (a, n))
        .collect();

    let mut grid = Grid::new(numbers.len(), numbers.len());
    for (a, b, d) in instructions {
        let a = numbers[a];
        let b = numbers[b];
        grid[(a, b)] += d;
        grid[(b, a)] += d;
    }
    grid
}

fn longest_circular(input: &Grid<isize>) -> isize {
    fn longest_circular_impl(
        input: &Grid<isize>,
        current: usize,
        to_visit: &mut Vec<usize>,
    ) -> isize {
        if to_visit.is_empty() {
            return input[(0, current)];
        }
        // recurse
        let mut max = isize::MIN;
        let len = to_visit.len();
        for i in 0..len {
            let next = to_visit.swap_remove(i);
            max = max.max(input[(current, next)] + longest_circular_impl(input, next, to_visit));
            to_visit.push(next);
            to_visit.swap(i, len - 1)
        }
        max
    }
    let mut to_visit = (1..input.rows()).collect_vec();
    longest_circular_impl(input, 0, &mut to_visit)
}

pub fn part1(input: &str) -> isize {
    let input = graph(parse(input));
    longest_circular(&input)
}

pub fn part2(input: &str) -> isize {
    let mut input = graph(parse(input));
    input.push_col(vec![0; input.rows()]);
    input.push_row(vec![0; input.cols()]);
    longest_circular(&input)
}
