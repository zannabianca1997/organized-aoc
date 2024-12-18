use std::usize;

use fnv::FnvHashSet;
use grid::Grid;

#[inline(always)]
fn parse(input: &str) -> (Grid<usize>, usize) {
    let mut grid = Grid::new(71, 71, usize::MAX);
    let mut lines = 0;
    for (p, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        grid[(x.parse().unwrap(), y.parse().unwrap())] = p;
        lines+=1;
    }
    (grid,lines)
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input).0;

    pathfind(&grid,1024).unwrap()
}

fn pathfind(grid: &Grid<usize>, t:usize) -> Option<usize> {
    let start = (0, 0);
    let end = (70, 70);

    let mut g_score = Grid::new_like(&grid, usize::MAX);
    g_score[start] = 0;

    let mut f_score = Grid::new_like(&grid, usize::MAX);
    f_score[start] = euristic(start, end);

    let mut open_set = FnvHashSet::from_iter([start]);

    while let Some(&pos) = open_set.iter().min_by_key(|s| f_score[**s]) {
        if pos == end {
            return Some(g_score[pos]);
        }

        open_set.remove(&pos);

        for n in [
            (pos.0 + 1, pos.1),
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|p| {
            // new point is inside the grid
            p.0 < 71 && p.1 < 71 
            // the byte of this point has not fallen yet
            && grid[*p] >= t
        })
        {
            let n_g_score = g_score[pos] + 1;
            if n_g_score < g_score[n] {
                g_score[n] = n_g_score;
                f_score[n] = n_g_score + euristic(n, end);
                open_set.insert(n);
            }
        }
    }

    None
}

fn euristic(start: (usize, usize), end: (usize, usize)) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

pub fn part2(input: &str) -> String {
    let (grid, len) = parse(input);

    let mut min = 0;
    let mut max = len+1;

    while min+1 < max {
        let middle = (min + max) / 2;
        if pathfind(&grid, middle).is_some() {
            min = middle
        }else {
            max = middle
        }
    }

    input.lines().nth(min).unwrap().to_owned()

}
