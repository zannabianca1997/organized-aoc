use std::fs::File;
use std::io::{BufWriter, Write};

use grid::Grid;
use lazy_regex::regex_captures;

fn parse(input: &str) -> impl Iterator<Item = ([i64; 2], [i64; 2])> + '_ {
    input.lines().map(|l| {
        let (_, px, py, vx, vy) = regex_captures!(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$", l).unwrap();
        (
            [px, py].map(|v| v.parse().unwrap()),
            [vx, vy].map(|v| v.parse().unwrap()),
        )
    })
}

pub fn part1<const WIDTH: i64, const HEIGHT: i64, const SIM_LEN: i64>(input: &str) -> usize {
    let mut quadrants: (usize, usize, usize, usize) = (0, 0, 0, 0);
    for ([px, py], [vx, vy]) in parse(input) {
        let pfx = (px + SIM_LEN * vx).rem_euclid(WIDTH);
        let pfy = (py + SIM_LEN * vy).rem_euclid(HEIGHT);
        match (pfx.cmp(&(WIDTH / 2)), pfy.cmp(&(HEIGHT / 2))) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants.0 += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants.1 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants.2 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants.3 += 1,
            (std::cmp::Ordering::Equal, _) | (_, std::cmp::Ordering::Equal) => (),
        }
    }
    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

pub fn quadrants<const WIDTH: i64, const HEIGHT: i64>(
    bots: &[([i64; 2], [i64; 2])],
    sim_len: i64,
) -> (usize, usize, usize, usize) {
    let mut quadrants: (usize, usize, usize, usize) = (0, 0, 0, 0);
    for ([px, py], [vx, vy]) in bots.iter().copied() {
        let pfx = (px + sim_len * vx).rem_euclid(WIDTH);
        let pfy = (py + sim_len * vy).rem_euclid(HEIGHT);
        match (pfx.cmp(&(WIDTH / 2)), pfy.cmp(&(HEIGHT / 2))) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants.0 += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants.1 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants.2 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants.3 += 1,
            (std::cmp::Ordering::Equal, _) | (_, std::cmp::Ordering::Equal) => (),
        }
    }
    quadrants
}

pub fn print_tree<const WIDTH: i64, const HEIGHT: i64>(
    mut f: impl Write,
    bots: &[([i64; 2], [i64; 2])],
    sim_len: i64,
) {
    let mut grid = Grid::new(HEIGHT as usize / 3 + 1, WIDTH as usize / 3 + 1, 0u8);

    for ([px, py], [vx, vy]) in bots.iter().copied() {
        let pfx = (px + sim_len * vx).rem_euclid(WIDTH);
        let pfy = (py + sim_len * vy).rem_euclid(HEIGHT);
        grid[(pfx as usize / 3, pfy as usize / 3)] += 1;
    }

    writeln!(f, "== Number of steps: {sim_len} ==\n").unwrap();

    for y in 0..grid.shape().1 {
        for x in 0..grid.shape().0 {
            let num = grid[(x, y)];
            f.write(&[if num != 0 { num + b'0' } else { b'.' }])
                .unwrap();
        }
        f.write(&[b'\n']).unwrap();
    }
}

pub fn part2(input: &str) -> u64 {
    let bots: Box<[_]> = parse(input).collect();

    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;

    (0..WIDTH * HEIGHT)
        .map(|sim_len| {
            let (a, b, c, d) = quadrants::<WIDTH, HEIGHT>(&bots, sim_len);
            let mut quadrants = [a, b, c, d];
            quadrants.sort();

            (sim_len, quadrants[3] - quadrants[2])
        })
        .max_by_key(|(_, diff)| *diff)
        .unwrap()
        .0 as _
}

#[test]
fn example() {
    assert_eq!(
        part1::<11, 7, 100>(
            r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        ),
        12
    )
}
