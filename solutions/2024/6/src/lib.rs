use grid::Grid;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space { visited: bool },
    Wall,
}

impl Cell {
    fn as_visited(&self) -> Option<&bool> {
        if let Self::Space { visited } = self {
            Some(visited)
        } else {
            None
        }
    }
    fn as_visited_mut(&mut self) -> Option<&mut bool> {
        if let Self::Space { visited } = self {
            Some(visited)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Right = 2,
    Down = 4,
    Left = 8,
}

fn parse(input: &str) -> (Grid<Cell>, ((usize, usize), Direction)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().filter(|l| !l.trim().is_empty()).count();
    let mut grid = Grid::new(height, width, Cell::Space { visited: false });
    let mut position = None;
    for (y, line) in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .rev()
        .enumerate()
    {
        for (x, ch) in line.chars().enumerate() {
            if '#' == ch {
                grid[(x, y)] = Cell::Wall
            }
            if ch == '^' {
                position = Some(((x, y), Direction::Up))
            }
        }
    }
    (grid, position.unwrap())
}

pub fn part1(input: &str) -> usize {
    let (mut grid, ((mut x, mut y), mut direction)) = parse(input);
    let mut visited = 0;
    loop {
        {
            let was_visited = grid.get_mut(x, y).unwrap().as_visited_mut().unwrap();
            if !*was_visited {
                visited += 1
            }
            *was_visited = true;
        }

        let (mut nx, mut ny) = update_pos(direction, x, y);
        'spin: loop {
            match grid.get(nx, ny) {
                Some(Cell::Space { .. }) => break 'spin,
                Some(Cell::Wall) => (),
                None => return visited,
            }
            direction = turn_right(direction);
            let new_npos = update_pos(direction, x, y);
            nx = new_npos.0;
            ny = new_npos.1
        }
        x = nx;
        y = ny;
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn update_pos(direction: Direction, x: usize, y: usize) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y.wrapping_sub(1)),
        Direction::Right => (x + 1, y),
        Direction::Left => (x.wrapping_sub(1), y),
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell2 {
    Space { visited: u8 },
    Wall,
}

impl Cell2 {
    fn as_visited_mut(&mut self) -> Option<&mut u8> {
        if let Self::Space { visited } = self {
            Some(visited)
        } else {
            None
        }
    }
}

fn parse2(input: &str) -> (Grid<Cell2>, ((usize, usize), Direction)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().filter(|l| !l.trim().is_empty()).count();
    let mut grid = Grid::new(height, width, Cell2::Space { visited: 0 });
    let mut position = None;
    for (y, line) in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .rev()
        .enumerate()
    {
        for (x, ch) in line.chars().enumerate() {
            if '#' == ch {
                grid[(x, y)] = Cell2::Wall
            }
            if ch == '^' {
                position = Some(((x, y), Direction::Up))
            }
        }
    }
    (grid, position.unwrap())
}

pub fn part2(input: &str) -> usize {
    let (grid, ((x, y), direction)) = parse2(input);
    let mut looping = 0;
    for (ox, oy, _) in PositionsIter::new(&grid, x, y, direction)
        .filter(|(px, py, visited_before)| !visited_before && (*px, *py) != (x, y))
    {
        let mut grid = grid.clone();
        grid[(ox, oy)] = Cell2::Wall;

        if is_looping(grid, x, y, direction) {
            looping += 1
        }
    }
    looping
}

struct PositionsIter {
    grid: Grid<Cell>,
    x: usize,
    y: usize,
    direction: Direction,
}
impl PositionsIter {
    fn new(grid: &Grid<Cell2>, x: usize, y: usize, direction: Direction) -> Self {
        Self {
            grid: grid.map_ref(|c| match c {
                Cell2::Space { visited } => Cell::Space {
                    visited: *visited != 0,
                },
                Cell2::Wall => Cell::Wall,
            }),
            x,
            y,
            direction,
        }
    }
}
impl Iterator for PositionsIter {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        *self
            .grid
            .get_mut(self.x, self.y)
            .unwrap()
            .as_visited_mut()
            .unwrap() = true;

        let (mut nx, mut ny) = update_pos(self.direction, self.x, self.y);
        'spin: loop {
            match self.grid.get(nx, ny) {
                Some(Cell::Space { .. }) => break 'spin,
                Some(Cell::Wall) => (),
                None => return None,
            }
            self.direction = turn_right(self.direction);
            let new_npos = update_pos(self.direction, self.x, self.y);
            nx = new_npos.0;
            ny = new_npos.1
        }
        self.x = nx;
        self.y = ny;
        Some((
            self.x,
            self.y,
            *self.grid.get(self.x, self.y).unwrap().as_visited().unwrap(),
        ))
    }
}

fn is_looping(mut grid: Grid<Cell2>, mut x: usize, mut y: usize, mut direction: Direction) -> bool {
    loop {
        {
            let was_visited = grid.get_mut(x, y).unwrap().as_visited_mut().unwrap();
            if *was_visited & direction as u8 != 0 {
                // we have been here before, and with the same direction
                return true;
            }
            *was_visited |= direction as u8;
        }

        let (mut nx, mut ny) = update_pos(direction, x, y);
        'spin: loop {
            match grid.get(nx, ny) {
                Some(Cell2::Space { .. }) => {
                    x = nx;
                    y = ny;
                    break 'spin;
                }
                Some(Cell2::Wall) => {
                    direction = turn_right(direction);
                    let new_npos = update_pos(direction, x, y);
                    nx = new_npos.0;
                    ny = new_npos.1
                }
                None => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn p1() {
        assert_eq!(part1(TEST_INPUT), 41)
    }
    #[test]
    fn p2() {
        assert_eq!(part2(TEST_INPUT), 6)
    }
}
