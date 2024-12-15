use grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty = 0,

    Wall,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(
    input: &str,
) -> (
    Grid<Cell>,
    (usize, usize),
    impl Iterator<Item = Direction> + '_,
) {
    let (grid_input, instructions) = input.split_once("\n\n").unwrap();

    let mut pos = None;

    let width = grid_input.lines().next().unwrap().len();
    let height = grid_input.lines().count();
    let mut grid = Grid::new(height, width, Cell::Empty);
    for (y, line) in grid_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::Box,
                '@' => {
                    pos = Some((x, y));
                    Cell::Empty
                }
                _ => panic!("Invalid grid cell: {ch:?}"),
            };
        }
    }
    let grid = grid;

    let pos = pos.unwrap();

    let instructions = instructions
        .chars()
        .filter(|c| *c != '\n')
        .map(|ch| match ch {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        });

    (grid, pos, instructions)
}

pub fn part1(input: &str) -> usize {
    let (mut grid, mut pos, instructions) = parse(input);

    for direction in instructions {
        let diff = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_pos = (
            pos.0.wrapping_add_signed(diff.0),
            pos.1.wrapping_add_signed(diff.1),
        );

        let mut non_box_space = new_pos;
        let mut pushing = false;
        while grid[non_box_space] == Cell::Box {
            non_box_space = (
                non_box_space.0.wrapping_add_signed(diff.0),
                non_box_space.1.wrapping_add_signed(diff.1),
            );
            pushing = true;
        }

        if grid[non_box_space] == Cell::Wall {
            continue; // This instruction is ignored
        }

        if pushing {
            grid[new_pos] = Cell::Empty;
            grid[non_box_space] = Cell::Box;
        }

        pos = new_pos
    }

    let mut total = 0;
    for y in 0..grid.shape().1 {
        for x in 0..grid.shape().0 {
            if grid[(x, y)] == Cell::Box {
                total += y * 100 + x
            }
        }
    }
    total
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell2 {
    Empty = 0,

    Wall,
    BoxLeft,
    BoxRight,
}

fn parse2(
    input: &str,
) -> (
    Grid<Cell2>,
    (usize, usize),
    impl Iterator<Item = Direction> + '_,
) {
    let (grid_input, instructions) = input.split_once("\n\n").unwrap();

    let mut pos = None;

    let width = 2 * grid_input.lines().next().unwrap().len();
    let height = grid_input.lines().count();
    let mut grid = Grid::new(height, width, Cell2::Empty);
    for (y, line) in grid_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(2 * x, y)] = match ch {
                '#' => Cell2::Wall,
                '.' => Cell2::Empty,
                'O' => Cell2::BoxLeft,
                '@' => {
                    pos = Some((2 * x, y));
                    Cell2::Empty
                }
                _ => panic!("Invalid grid cell: {ch:?}"),
            };
            grid[(2 * x + 1, y)] = match ch {
                '#' => Cell2::Wall,
                '.' => Cell2::Empty,
                'O' => Cell2::BoxRight,
                '@' => Cell2::Empty,
                _ => panic!("Invalid grid cell: {ch:?}"),
            };
        }
    }
    let grid = grid;

    let pos = pos.unwrap();

    let instructions = instructions
        .chars()
        .filter(|c| *c != '\n')
        .map(|ch| match ch {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        });

    (grid, pos, instructions)
}

pub fn part2(input: &str) -> usize {
    let (mut grid, mut pos, instructions) = parse2(input);
    let mut moves = vec![];
    let mut examinees = vec![];
    let mut examined = fnv::FnvHashSet::default();

    'directions: for direction in instructions {
        let diff = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_pos = (
            pos.0.wrapping_add_signed(diff.0),
            pos.1.wrapping_add_signed(diff.1),
        );

        if grid[new_pos] != Cell2::Empty {
            moves.clear();
            examinees.clear();
            examined.clear();

            examinees.push(new_pos);
            while let Some(examinee) = examinees.pop() {
                if !examined.insert(examinee) {
                    continue;
                }

                match grid[examinee] {
                    Cell2::Empty => continue,
                    Cell2::Wall => continue 'directions,
                    Cell2::BoxLeft => examinees.push((examinee.0 + 1, examinee.1)),
                    Cell2::BoxRight => examinees.push((examinee.0 - 1, examinee.1)),
                }

                let examinee_new_pos = (
                    examinee.0.wrapping_add_signed(diff.0),
                    examinee.1.wrapping_add_signed(diff.1),
                );

                moves.push((examinee, examinee_new_pos));
                examinees.push(examinee_new_pos);
            }

            // Sorting the moves so they do not overlap
            moves.sort_unstable_by_key(|(old_pos, _)| {
                -(old_pos.0 as isize * diff.0 + old_pos.1 as isize * diff.1)
            });

            for (old_pos, new_pos) in moves.drain(..) {
                grid[new_pos] = grid[old_pos];
                grid[old_pos] = Cell2::Empty
            }
        }

        #[cfg(test)]
        for y in 0..grid.shape().1 {
            for x in 0..grid.shape().0 {
                if (x, y) == new_pos {
                    print!("@");
                    continue;
                }
                match grid[(x, y)] {
                    Cell2::Empty => print!("."),
                    Cell2::Wall => print!("#"),
                    Cell2::BoxLeft => print!("["),
                    Cell2::BoxRight => print!("]"),
                }
            }
            println!();
        }

        pos = new_pos
    }

    let mut total = 0;
    for y in 0..grid.shape().1 {
        for x in 0..grid.shape().0 {
            if grid[(x, y)] == Cell2::BoxLeft {
                total += y * 100 + x
            }
        }
    }
    total
}

#[test]
fn example() {
    assert_eq!(
        part2(
            r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        ),
        9021
    )
}

#[test]
fn edge_case_1() {
    part2(
        r"#######
#.....#
#.OOO.#
#..OO@#
#..O..#
#.....#
#######

<vv<<^
",
    );
}
