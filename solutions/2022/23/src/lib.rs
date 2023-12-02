use grid::Grid;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Proposal {
    #[default]
    None,
    Some(usize, usize),
    Blocked,
}

fn parse_input(map: &str) -> Grid<bool> {
    let height = map.lines().count();
    let width = map.lines().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = Grid::new(height, width);
    for (row, col, ch) in map.lines().enumerate().flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .map(move |(col, ch)| (row, col, ch))
    }) {
        grid[(row, col)] = match ch {
            '.' => false,
            '#' => true,
            ch => panic!("{ch} is not a valid char"),
        }
    }

    grid
}

fn expand<T: Default + Copy>(grid: Grid<T>, border: usize) -> Grid<T> {
    let new_rows = grid.rows() + border * 2;
    let new_cols = grid.cols() + border * 2;

    let mut new_grid = Grid::new(new_rows, new_cols);
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            new_grid[(border + row, border + col)] = grid[(row, col)]
        }
    }

    new_grid
}

pub fn part1(input: &str) -> usize {
    let mut grid = expand(parse_input(input), 10);
    let mut proposals: Grid<Proposal> = Grid::new(grid.rows(), grid.cols());
    /*println!("== Initial State ==");
    print_grid(&grid);*/
    for i in 0..10 {
        step(&mut grid, &mut proposals, i);
    }
    count_empty(&grid, borders(&grid))
}

fn step(grid: &mut Grid<bool>, proposals: &mut Grid<Proposal>, step: usize) -> bool {
    // first half
    for row in 0..grid.rows() {
        'col: for col in 0..grid.cols() {
            if grid[(row, col)] {
                debug_assert!(
                    1 <= row && row < grid.rows() - 1 && 1 <= col && col < grid.cols() - 1
                );
                // check for neighbours
                let mut neighbours = false;
                for i in [row - 1, row, row + 1] {
                    for j in [col - 1, col, col + 1] {
                        if (i, j) != (row, col) && grid[(i, j)] {
                            neighbours = true
                        }
                    }
                }
                if !neighbours {
                    continue 'col;
                }
                // propose
                let p_list = [
                    // North
                    (
                        [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)],
                        (row - 1, col),
                    ),
                    // South
                    (
                        [(row + 1, col - 1), (row + 1, col), (row + 1, col + 1)],
                        (row + 1, col),
                    ),
                    // West
                    (
                        [(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)],
                        (row, col - 1),
                    ),
                    // East
                    (
                        [(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)],
                        (row, col + 1),
                    ),
                ];

                let mut proposal = None;

                'check_proposals: for j in 0..4 {
                    let (check_list, dest) = &p_list[(step + j) % 4];
                    if check_list.iter().all(|(row, col)| !grid[(*row, *col)]) {
                        proposal = Some(*dest);
                        break 'check_proposals;
                    };
                }

                if let Some((p_row, p_col)) = proposal {
                    use Proposal::*;
                    proposals[(p_row, p_col)] = match proposals[(p_row, p_col)] {
                        None => Some(row, col),
                        Some(_, _) => Blocked,
                        Blocked => Blocked,
                    }
                }
            }
        }
    }
    // second half
    let mut something_moved = false;
    for row in 0..proposals.rows() {
        for col in 0..proposals.cols() {
            if let Proposal::Some(s_row, s_col) = proposals[(row, col)] {
                debug_assert!(grid[(s_row, s_col)]);
                debug_assert!(!grid[(row, col)]);
                // move elf
                grid[(s_row, s_col)] = false;
                grid[(row, col)] = true;

                something_moved = true;
            }
        }
    }
    // clean up
    proposals.fill(Proposal::None);

    /*println!("== End of round {} ==", i + 1);
    print_grid(&grid);*/
    something_moved
}

fn borders(grid: &Grid<bool>) -> (usize, usize, usize, usize) {
    let mut min_row = grid.rows();
    let mut max_row = 0;
    let mut min_col = grid.cols();
    let mut max_col = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] {
                min_row = min_row.min(row);
                max_row = max_row.max(row + 1);
                min_col = min_col.min(col);
                max_col = max_col.max(col + 1);
            }
        }
    }
    (min_row, max_row, min_col, max_col)
}

fn count_empty(grid: &Grid<bool>, rect: (usize, usize, usize, usize)) -> usize {
    let (min_row, max_row, min_col, max_col) = rect;
    let mut count = 0;
    for row in min_row..max_row {
        for col in min_col..max_col {
            if !grid[(row, col)] {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut proposals: Grid<Proposal> = Grid::new(grid.rows(), grid.cols());
    /*println!("== Initial State ==");
    print_grid(&grid);*/
    for i in 0.. {
        // check if we need to enlarge the grid
        if grid
            .iter_row(0)
            .chain(grid.iter_row(grid.rows() - 1))
            .chain(grid.iter_col(0))
            .chain(grid.iter_col(grid.cols() - 1))
            .any(|t| *t)
        {
            grid = expand(grid, 10);
            proposals = Grid::new(grid.rows(), grid.cols());
        }
        // step the simulation
        if !step(&mut grid, &mut proposals, i) {
            return i + 1;
        };
    }
    unreachable!()
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<bool>) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            print!(
                "{}",
                match grid[(row, col)] {
                    false => '.',
                    true => '#',
                }
            )
        }
        println!()
    }
}
