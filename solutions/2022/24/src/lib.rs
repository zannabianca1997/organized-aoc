use std::collections::BinaryHeap;

use arrayvec::ArrayVec;
use grid::Grid;

fn parse_input(
    input: &str,
) -> (
    (usize, usize),
    (Grid<bool>, Grid<bool>, Grid<bool>, Grid<bool>),
) {
    let rows = input.trim().lines().count() - 2;
    let cols = input
        .trim()
        .lines()
        .map(|line| line.trim().len() - 2)
        .max()
        .unwrap();
    let [mut blizzard_up, mut blizzard_right, mut blizzard_down, mut blizzard_left] =
        [(); 4].map(|_| Grid::new(rows, cols));

    for (row, line) in input.trim().lines().skip(1).take(rows).enumerate() {
        for (col, ch) in line.chars().skip(1).take(cols).enumerate() {
            match ch {
                '^' => blizzard_up[(row, col)] = true,
                '>' => blizzard_right[(row, col)] = true,
                'v' => blizzard_down[(row, col)] = true,
                '<' => blizzard_left[(row, col)] = true,
                '.' => (),
                _ => panic!("{ch} is not a valid input char"),
            }
        }
    }

    (
        (rows, cols),
        (blizzard_up, blizzard_right, blizzard_down, blizzard_left),
    )
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileState {
    Unexamined,
    Blizzard,
    Examined,
}

pub fn part1(input: &str) -> usize {
    let ((rows, cols), (blizzard_up, blizzard_right, blizzard_down, blizzard_left)) =
        parse_input(input);

    // blizzards are periodic, and so is the state space
    let period = lcm(rows, cols);

    // create the possible state space memory
    let mut state_space: Box<[TileState]> =
        vec![TileState::Unexamined; period * rows * cols].into_boxed_slice();
    // fill in the blizzards
    for p in 0..period {
        for r in 0..rows {
            for c in 0..cols {
                if blizzard_down[((period.div_ceil(rows) * rows + r - p) % rows, c)]
                    || blizzard_up[((r + p) % rows, c)]
                    || blizzard_right[(r, (period.div_ceil(cols) * cols + c - p) % cols)]
                    || blizzard_left[(r, (c + p) % cols)]
                {
                    state_space[(p * rows + r) * cols + c] = TileState::Blizzard;
                }
            }
        }
    }

    // euristic for the A*
    let euristic = |_p: usize, r: usize, c: usize| -> usize { 1 + (rows - 1 - r) + (cols - 1 - c) };

    // create the states queue
    let mut states_to_examine = BinaryHeap::new();

    // adding the possible enter states
    for enter in 1..=period {
        if state_space[(enter % period * rows + 0) * cols + 0] != TileState::Blizzard {
            states_to_examine.push((
                -((enter + euristic(enter % period, 0, 0)) as isize),
                (enter, 0usize, 0usize),
            ))
        }
    }

    while let Some((_, (m, r, c))) = states_to_examine.pop() {
        debug_assert_ne!(
            state_space[(m % period * rows + r) * cols + c],
            TileState::Blizzard
        );
        // did we arrive?
        if r == rows - 1 && c == cols - 1 {
            // adding 1 to count the exit step
            return m + 1;
        }
        // pruning
        if state_space[(m % period * rows + r) * cols + c] == TileState::Examined {
            // someone was already here. Pruning...
            continue;
        }
        // mark as examined
        state_space[(m % period * rows + r) * cols + c] = TileState::Examined;

        // enumerate the neighbours
        let mut neighbours: ArrayVec<_, 5> = ArrayVec::new();
        neighbours.push((m + 1, r, c));
        if r > 0 {
            neighbours.push((m + 1, r - 1, c))
        }
        if r < rows - 1 {
            neighbours.push((m + 1, r + 1, c))
        }
        if c > 0 {
            neighbours.push((m + 1, r, c - 1))
        }
        if c < cols - 1 {
            neighbours.push((m + 1, r, c + 1))
        }

        // filtering the neighbours and examine the path extension
        for (m, r, c) in neighbours {
            // is this a new possible branch?
            if state_space[(m % period * rows + r) * cols + c] == TileState::Unexamined {
                // add the path to the one needing further examining
                states_to_examine.push((-((m + euristic(m % period, r, c)) as isize), (m, r, c)))
            }
        }
    }

    // we hit dead end everywhere...
    panic!("No path found...")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileState2 {
    Blizzard,
    Space([bool; 3]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Traverse {
    First = 0,
    Back = 1,
    Return = 2,
}

pub fn part2(input: &str) -> usize {
    let ((rows, cols), (blizzard_up, blizzard_right, blizzard_down, blizzard_left)) =
        parse_input(input);

    // blizzards are periodic, and so is the state space
    let period = lcm(rows, cols);

    // create the possible state space memory
    let mut state_space: Box<[TileState2]> =
        vec![TileState2::Space([false; 3]); period * rows * cols].into_boxed_slice();
    // fill in the blizzards
    for p in 0..period {
        for r in 0..rows {
            for c in 0..cols {
                if blizzard_down[((period.div_ceil(rows) * rows + r - p) % rows, c)]
                    || blizzard_up[((r + p) % rows, c)]
                    || blizzard_right[(r, (period.div_ceil(cols) * cols + c - p) % cols)]
                    || blizzard_left[(r, (c + p) % cols)]
                {
                    state_space[(p * rows + r) * cols + c] = TileState2::Blizzard;
                }
            }
        }
    }

    // euristic for the A*
    let euristic = |_p: usize, r: usize, c: usize, traverse: Traverse| -> usize {
        match traverse {
            Traverse::First => 2 * (rows - 1 + cols - 1 + 2) + 1 + (rows - 1 - r) + (cols - 1 - c),
            Traverse::Back => (rows - 1 + cols - 1 + 2) + 1 + r + c,
            Traverse::Return => 1 + (rows - 1 - r) + (cols - 1 - c),
        }
    };

    // create the states queue
    let mut states_to_examine = BinaryHeap::new();

    // adding the possible enter states
    for enter in 1..=period {
        if state_space[(enter % period * rows + 0) * cols + 0] != TileState2::Blizzard {
            states_to_examine.push((
                -((enter + euristic(enter % period, 0, 0, Traverse::First)) as isize),
                (enter, 0usize, 0usize, Traverse::First),
            ))
        }
    }

    while let Some((_, (m, r, c, traverse))) = states_to_examine.pop() {
        // did we arrive?
        if r == rows - 1 && c == cols - 1 && traverse == Traverse::Return {
            // adding 1 to count the exit step
            return m + 1;
        }
        // pruning
        match &mut state_space[(m % period * rows + r) * cols + c] {
            TileState2::Space(v) if v[traverse as usize] => continue, // somebody was there before, in the same traverse
            TileState2::Space(v) => v[traverse as usize] = true,      // mark as examined
            TileState2::Blizzard => unreachable!(),
        }

        // enumerate the neighbours
        let mut neighbours: ArrayVec<_, 5> = ArrayVec::new();
        neighbours.push((m + 1, r, c, traverse));
        if r > 0 {
            neighbours.push((m + 1, r - 1, c, traverse))
        }
        if r < rows - 1 {
            neighbours.push((m + 1, r + 1, c, traverse))
        }
        if c > 0 {
            neighbours.push((m + 1, r, c - 1, traverse))
        }
        if c < cols - 1 {
            neighbours.push((m + 1, r, c + 1, traverse))
        }
        // adding the first stop
        let neighbours = neighbours.into_iter().chain(
            (traverse == Traverse::First && r == rows - 1 && c == cols - 1)
                .then(|| ((m + 2)..(m + 2 + period)).map(|m| (m, r, c, Traverse::Back)))
                .into_iter()
                .flatten(),
        );
        // adding the second stop
        let neighbours = neighbours.chain(
            (traverse == Traverse::Back && r == 0 && c == 0)
                .then(|| ((m + 2)..(m + 2 + period)).map(|m| (m, r, c, Traverse::Return)))
                .into_iter()
                .flatten(),
        );

        // filtering the neighbours and examine the path extension
        for (m, r, c, traverse) in neighbours {
            // is this a new possible branch?
            if let TileState2::Space(v) = &state_space[(m % period * rows + r) * cols + c] {
                if !v[traverse as usize] {
                    // add the path to the one needing further examining
                    states_to_examine.push((
                        -((m + euristic(m % period, r, c, traverse)) as isize),
                        (m, r, c, traverse),
                    ))
                }
            }
        }
    }

    // we hit dead end everywhere...
    panic!("No path found...")
}
