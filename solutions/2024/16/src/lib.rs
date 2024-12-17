use std::{
    cmp::{
        Ordering::{Equal, Greater},
        Reverse,
    },
    collections::hash_map::Entry,
    iter::repeat,
};

use fnv::{FnvHashMap, FnvHashSet};
use grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty = 0,

    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (Grid<Cell>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid = Grid::new(height, width, Cell::Empty);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'S' => {
                    start = Some((x, y));
                    Cell::Empty
                }
                'E' => {
                    end = Some((x, y));
                    Cell::Empty
                }
                _ => panic!("Invalid grid cell: {ch:?}"),
            };
        }
    }
    (grid, start.unwrap(), end.unwrap())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}
#[derive(Debug, Clone, Copy, Hash)]
struct StateInfo {
    cost: usize,
    estimate: usize,
}

pub fn part1(input: &str) -> usize {
    let (grid, start, end) = parse(input);

    let start_state = State {
        pos: start,
        dir: Direction::Right,
    };

    let mut openset = FnvHashMap::from_iter([(
        start_state,
        StateInfo {
            cost: 0,
            estimate: euristic(start_state, end, &grid),
        },
    )]);

    let mut closeset = FnvHashMap::from_iter([(start_state, 0)]);

    while let Some((state @ State { pos, dir }, StateInfo { cost, .. })) = {
        match openset.iter().min_by_key(|v| v.1.estimate) {
            Some((&s, _)) => openset.remove_entry(&s),
            None => None,
        }
    } {
        if state.pos == end {
            return cost;
        }

        let npos = match dir {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };

        for (neighbour, n_cost) in (grid[npos] == Cell::Empty)
            .then_some((State { pos: npos, dir }, cost + 1))
            .into_iter()
            .chain(
                [
                    State {
                        pos,
                        dir: match dir {
                            Direction::Up | Direction::Down => Direction::Left,
                            Direction::Left | Direction::Right => Direction::Up,
                        },
                    },
                    State {
                        pos,
                        dir: match dir {
                            Direction::Up | Direction::Down => Direction::Right,
                            Direction::Left | Direction::Right => Direction::Down,
                        },
                    },
                ]
                .into_iter()
                .zip(repeat(cost + 1000)),
            )
        {
            if closeset.get(&neighbour).is_none_or(|c| *c > n_cost) {
                closeset.insert(neighbour, n_cost);

                openset.insert(
                    neighbour,
                    StateInfo {
                        cost: n_cost,
                        estimate: n_cost + euristic(neighbour, end, &grid),
                    },
                );
            }
        }
    }

    panic!("No way to the end")
}

#[derive(Debug, Clone)]
struct StateInfo2 {
    cost: usize,
    estimate: usize,
    reached_from: fnv::FnvHashSet<State>,
}

fn euristic(State { pos, dir }: State, end: (usize, usize), _grid: &Grid<Cell>) -> usize {
    use std::cmp::Ordering::*;
    use Direction::*;
    // Trivial euristic: taxi driver
    pos.0.abs_diff(end.0)
        + pos.1.abs_diff(end.1)
        // Count of minimum turn arounds
        + 1000
            * match (dir, pos.0.cmp(&end.0), pos.1.cmp(&end.1)) {
                (_, Equal, Equal) => 0,

                (Up, Less | Greater, Less) => 2,
                (Up, Less | Greater, Equal | Greater) => 1,
                (Up, Equal, Less) => 2,
                (Up, Equal, Greater) => 0,
                (Down, Less | Greater, Less | Equal) => 1,
                (Down, Less | Greater, Greater) => 2,
                (Down, Equal, Less) => 0,
                (Down, Equal, Greater) => 2,

                (Left, Less, Less | Greater) => 2,
                (Left, Equal | Greater, Less | Greater) => 1,
                (Left, Less, Equal) => 2,
                (Left, Greater, Equal) => 0,
                (Right, Less | Equal, Less | Greater) => 1,
                (Right, Greater, Less | Greater) => 2,
                (Right, Less, Equal) => 0,
                (Right, Greater, Equal) => 2,
            }
}

pub fn part2(input: &str) -> usize {
    let (grid, start, end) = parse(input);

    let start_state = State {
        pos: start,
        dir: Direction::Right,
    };

    let mut openset = FnvHashMap::from_iter([(
        start_state,
        StateInfo {
            cost: 0,
            estimate: euristic(start_state, end, &grid),
        },
    )]);

    // let mut closeset = FnvHashMap::from_iter([(start_state, (0, FnvHashSet::default()))]);

    while let Some((state @ State { pos, dir }, StateInfo { cost, .. })) = {
        match openset.iter().min_by_key(|v| v.1.estimate) {
            Some((&s, _)) => openset.remove_entry(&s),
            None => None,
        }
    } {
        if state.pos == end {
            return cost;
        }

        let npos = match dir {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };

        for (neighbour, n_cost) in (grid[npos] == Cell::Empty)
            .then_some((State { pos: npos, dir }, cost + 1))
            .into_iter()
            .chain(
                [
                    State {
                        pos,
                        dir: match dir {
                            Direction::Up | Direction::Down => Direction::Left,
                            Direction::Left | Direction::Right => Direction::Up,
                        },
                    },
                    State {
                        pos,
                        dir: match dir {
                            Direction::Up | Direction::Down => Direction::Right,
                            Direction::Left | Direction::Right => Direction::Down,
                        },
                    },
                ]
                .into_iter()
                .zip(repeat(cost + 1000)),
            )
        {
            /*  if closeset.get(&neighbour).is_none_or(|c| c.0 >= n_cost) {
                match closeset.entry(neighbour) {
                    Entry::Occupied(occupied_entry) if occupied_entry.get().0 == n_cost => {
                        todo!()
                    }
                    Entry::Vacant(vacant_entry) => todo!(),
                }

                openset.insert(
                    neighbour,
                    StateInfo {
                        cost: n_cost,
                        estimate: n_cost + euristic(neighbour, end, &grid),
                    },
                );
            } */
        }
    }

    panic!("No way to the end")
}
