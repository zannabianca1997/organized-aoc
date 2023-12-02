use std::io::{stdin, stdout};

use grid::Grid;
use termion::{input::TermRead, raw::IntoRawMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    LEFT,
    RIGHT,
    AROUND,
    Nothing,
}
impl Turn {
    const fn reverse(self) -> Self {
        use Turn::*;
        match self {
            LEFT => RIGHT,
            RIGHT => LEFT,
            AROUND => AROUND,
            Nothing => Nothing,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}
impl Direction {
    const fn turned(self, turn: Turn) -> Self {
        use Direction::*;
        match (self, turn) {
            (LEFT, Turn::LEFT) => DOWN,
            (LEFT, Turn::RIGHT) => UP,
            (RIGHT, Turn::LEFT) => UP,
            (RIGHT, Turn::RIGHT) => DOWN,
            (UP, Turn::LEFT) => LEFT,
            (UP, Turn::RIGHT) => RIGHT,
            (DOWN, Turn::LEFT) => RIGHT,
            (DOWN, Turn::RIGHT) => LEFT,
            (LEFT, Turn::AROUND) => RIGHT,
            (RIGHT, Turn::AROUND) => LEFT,
            (UP, Turn::AROUND) => DOWN,
            (DOWN, Turn::AROUND) => UP,
            (LEFT, Turn::Nothing) => LEFT,
            (RIGHT, Turn::Nothing) => RIGHT,
            (UP, Turn::Nothing) => UP,
            (DOWN, Turn::Nothing) => DOWN,
        }
    }
    const fn facing(&self) -> usize {
        match self {
            Direction::RIGHT => 0,
            Direction::DOWN => 1,
            Direction::LEFT => 2,
            Direction::UP => 3,
        }
    }

    pub fn shift(&self, (mut row, mut col): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::RIGHT => col += 1,
            Direction::LEFT => col -= 1,
            Direction::DOWN => row += 1,
            Direction::UP => row -= 1,
        }
        (row, col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
    direction: Direction,
}
impl Pos {
    const fn stepped(&self) -> Self {
        Self {
            row: match self.direction {
                Direction::LEFT | Direction::RIGHT => self.row,
                Direction::UP => self.row - 1,
                Direction::DOWN => self.row + 1,
            },
            col: match self.direction {
                Direction::LEFT => self.col - 1,
                Direction::RIGHT => self.col + 1,
                Direction::UP | Direction::DOWN => self.col,
            },
            direction: self.direction,
        }
    }
    const fn turned(&self, turn: Turn) -> Self {
        Self {
            row: self.row,
            col: self.col,
            direction: self.direction.turned(turn),
        }
    }
    const fn coord(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Tile {
    Space,
    Wall,
    #[default]
    Extern,
    Warp(Box<WarpEnters>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct WarpEnters {
    up: Option<(usize, usize, Turn)>,
    down: Option<(usize, usize, Turn)>,
    left: Option<(usize, usize, Turn)>,
    right: Option<(usize, usize, Turn)>,
}
impl WarpEnters {
    fn enter(&self, direction: Direction) -> Option<&(usize, usize, Turn)> {
        match direction {
            Direction::LEFT => &self.left,
            Direction::RIGHT => &self.right,
            Direction::UP => &self.up,
            Direction::DOWN => &self.down,
        }
        .as_ref()
    }
}

fn parse_input(input: &str) -> ((Pos, Grid<Tile>), (Box<[usize]>, Box<[Turn]>)) {
    let (map, path) = input
        .split_once("\n\n")
        .expect("Missing empty separator line");

    (parse_map(map), parse_path(path))
}

fn parse_path(path: &str) -> (Box<[usize]>, Box<[Turn]>) {
    let segments: Box<[usize]> = path
        .trim()
        .split(&['R', 'L'])
        .map(|s| s.parse().unwrap())
        .collect();
    let turns: Box<[Turn]> = path
        .trim()
        .chars()
        .filter_map(|ch| match ch {
            'R' => Some(Turn::RIGHT),
            'L' => Some(Turn::LEFT),
            _ => None,
        })
        .collect();
    debug_assert!(segments.len() == turns.len() + 1);
    (segments, turns)
}

fn parse_map(map: &str) -> (Pos, Grid<Tile>) {
    let height = map.lines().count() + 2;
    let width = map.lines().map(|l| l.len()).max().unwrap_or(0) + 2;

    let mut grid = Grid::new(height, width);

    let cells = map.lines().enumerate().flat_map(|(r, line)| {
        line.chars().enumerate().map(move |(c, ch)| {
            (
                (r + 1, c + 1),
                match ch {
                    ' ' => Tile::Extern,
                    '#' => Tile::Wall,
                    '.' => Tile::Space,
                    ch => panic!("Unrecognized char {ch}"),
                },
            )
        })
    });

    let mut start = None;

    for ((row, column), tile) in cells {
        if tile == Tile::Space && start.is_none() {
            start = Some((row, column))
        }
        grid[(row, column)] = tile;
    }

    let start = start.expect("Grid is empty!");
    let pos = Pos {
        row: start.0,
        col: start.1,
        direction: Direction::RIGHT,
    };

    (pos, grid)
}

fn advance(mut pos: Pos, len: usize, field: &Grid<Tile>) -> Pos {
    for _ in 0..len {
        let Pos {
            row: new_row,
            col: new_col,
            direction,
        } = pos.stepped();
        match field.get(new_row, new_col) {
            Some(Tile::Space) => {
                pos = pos.stepped(); // no obstacles
            }
            Some(Tile::Wall) => {
                return pos; // hit a wall
            }
            Some(Tile::Warp(warp)) => {
                if let Some((new_row, new_col, turn)) = warp.enter(direction) {
                    // teleported
                    pos = Pos {
                        row: *new_row,
                        col: *new_col,
                        direction: direction.turned(*turn),
                    }
                } else {
                    // hit a wall
                    return pos;
                }
            }
            Some(Tile::Extern) | None => panic!("The field should be surronded by warps"),
        }
    }
    pos
}

fn put_wrapping_warps(mut field: Grid<Tile>) -> Grid<Tile> {
    // orizontal warps
    for row in 0..field.rows() {
        // warps from right side to left side

        let mut col = 0;
        // go to the field
        while let Some(Tile::Extern | Tile::Warp(_)) = field.get(row, col) {
            col += 1;
        }
        let warp_dest =
            (field.get(row, col) == Some(&Tile::Space)).then(|| (row, col, Turn::Nothing));
        // skip the field
        while let Some(Tile::Space | Tile::Wall) = field.get(row, col) {
            col += 1;
        }
        // check it was not an empty row
        if let Some(tile) = field.get_mut(row, col) {
            *tile = match tile {
                Tile::Space | Tile::Wall => unreachable!(),
                Tile::Extern => Tile::Warp(Box::new(WarpEnters {
                    right: warp_dest,
                    ..Default::default() // fill with Nones
                })),
                Tile::Warp(warp) => Tile::Warp(Box::new(WarpEnters {
                    right: warp_dest,
                    ..**warp // fill from the old values
                })),
            }
        }

        // warps from left side to right side

        let mut col = field.cols() - 1;
        // go to the field
        while let Some(Tile::Extern | Tile::Warp(_)) = field.get(row, col) {
            col -= 1;
        }
        let warp_dest =
            (field.get(row, col) == Some(&Tile::Space)).then(|| (row, col, Turn::Nothing));
        // skip the field
        while let Some(Tile::Space | Tile::Wall) = field.get(row, col) {
            col -= 1;
        }
        // check it was not an empty row
        if let Some(tile) = field.get_mut(row, col) {
            *tile = match tile {
                Tile::Space | Tile::Wall => unreachable!(),
                Tile::Extern => Tile::Warp(Box::new(WarpEnters {
                    left: warp_dest,
                    ..Default::default() // fill with Nones
                })),
                Tile::Warp(warp) => Tile::Warp(Box::new(WarpEnters {
                    left: warp_dest,
                    ..**warp // fill from the old values
                })),
            }
        }
    }

    // vertical warps
    for col in 0..field.cols() {
        // warps from bottom side to up side

        let mut row = 0;
        // go to the field
        while let Some(Tile::Extern | Tile::Warp(_)) = field.get(row, col) {
            row += 1;
        }
        let warp_dest =
            (field.get(row, col) == Some(&Tile::Space)).then(|| (row, col, Turn::Nothing));
        // skip the field
        while let Some(Tile::Space | Tile::Wall) = field.get(row, col) {
            row += 1;
        }
        // check it was not an empty col
        if let Some(tile) = field.get_mut(row, col) {
            *tile = match tile {
                Tile::Space | Tile::Wall => unreachable!(),
                Tile::Extern => Tile::Warp(Box::new(WarpEnters {
                    down: warp_dest,
                    ..Default::default() // fill with Nones
                })),
                Tile::Warp(warp) => Tile::Warp(Box::new(WarpEnters {
                    down: warp_dest,
                    ..**warp // fill from the old values
                })),
            }
        }

        // warps from bottom side to up side

        let mut row = field.rows() - 1;
        // go to the field
        while let Some(Tile::Extern | Tile::Warp(_)) = field.get(row, col) {
            row -= 1;
        }
        let warp_dest =
            (field.get(row, col) == Some(&Tile::Space)).then(|| (row, col, Turn::Nothing));
        // skip the field
        while let Some(Tile::Space | Tile::Wall) = field.get(row, col) {
            row -= 1;
        }
        // check it was not an empty col
        if let Some(tile) = field.get_mut(row, col) {
            *tile = match tile {
                Tile::Space | Tile::Wall => unreachable!(),
                Tile::Extern => Tile::Warp(Box::new(WarpEnters {
                    up: warp_dest,
                    ..Default::default() // fill with Nones
                })),
                Tile::Warp(warp) => Tile::Warp(Box::new(WarpEnters {
                    up: warp_dest,
                    ..**warp // fill from the old values
                })),
            }
        }
    }

    field
}

fn line(
    (start_row, start_col): (usize, usize),
    dir: Direction,
    len: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (0..len).map(move |d| match dir {
        Direction::RIGHT => (start_row, start_col + d),
        Direction::DOWN => (start_row + d, start_col),
        Direction::LEFT => (start_row, start_col - d),
        Direction::UP => (start_row - d, start_col),
    })
}

fn put_warp_line(
    field: &mut Grid<Tile>,
    positions: ((usize, usize), Direction),
    dests: ((usize, usize), Direction),
    len: usize,
    enter_direction: Direction,
    turn: Turn,
) {
    for (warp_pos, warp_dest) in
        line(positions.0, positions.1, len).zip(line(dests.0, dests.1, len))
    {
        /*
        if field.get(warp_dest.0, warp_dest.1).is_none()
            || field.get(warp_pos.0, warp_pos.1).is_none()
        {
            return;
        }*/
        let warp_dest = (field
            .get(warp_dest.0, warp_dest.1)
            .expect("Destinations should be inside the field")
            == &Tile::Space)
            .then(|| (warp_dest.0, warp_dest.1, turn));
        let warp_tile = field
            .get_mut(warp_pos.0, warp_pos.1)
            .expect("Positions should be inside the field");
        use Direction::*;
        *warp_tile = match (&warp_tile, enter_direction) {
            (Tile::Space | Tile::Wall, _) => panic!("Cannot place a warp on the field"),
            (Tile::Extern, UP) => Tile::Warp(Box::new(WarpEnters {
                up: warp_dest,
                ..Default::default() // fill with Nones
            })),
            (Tile::Extern, LEFT) => Tile::Warp(Box::new(WarpEnters {
                left: warp_dest,
                ..Default::default() // fill with Nones
            })),
            (Tile::Extern, RIGHT) => Tile::Warp(Box::new(WarpEnters {
                right: warp_dest,
                ..Default::default() // fill with Nones
            })),
            (Tile::Extern, DOWN) => Tile::Warp(Box::new(WarpEnters {
                down: warp_dest,
                ..Default::default() // fill with Nones
            })),
            (Tile::Warp(warp), UP) => Tile::Warp(Box::new(WarpEnters {
                up: warp_dest,
                ..**warp // fill from the old values
            })),
            (Tile::Warp(warp), DOWN) => Tile::Warp(Box::new(WarpEnters {
                down: warp_dest,
                ..**warp // fill from the old values
            })),
            (Tile::Warp(warp), LEFT) => Tile::Warp(Box::new(WarpEnters {
                left: warp_dest,
                ..**warp // fill from the old values
            })),
            (Tile::Warp(warp), RIGHT) => Tile::Warp(Box::new(WarpEnters {
                right: warp_dest,
                ..**warp // fill from the old values
            })),
        };
    }
}

fn put_coupled_warp_lines(
    field: &mut Grid<Tile>,
    a: ((usize, usize), Direction),
    b: ((usize, usize), Direction),
    len: usize,
    a_enter_direction: Direction,
    b_enter_direction: Direction,
    turn: Turn,
) {
    put_warp_line(
        field,
        (a_enter_direction.shift(a.0), a.1),
        b,
        len,
        a_enter_direction,
        turn,
    );
    put_warp_line(
        field,
        (b_enter_direction.shift(b.0), b.1),
        a,
        len,
        b_enter_direction,
        turn.reverse(),
    );
}

fn put_cube_warps(mut field: Grid<Tile>) -> Grid<Tile> {
    assert!(
        field.size() == (200 + 2, 150 + 2),
        "Different field sizes are unimplemented"
    );

    /*
              G   F
            +---+---+
            |   |   |
           D|   |   |E
            |   |   |
            +---+---+
            |   | C
           B|   |C
          B |   |
        +---+---+
        |   |   |
       D|   |   |E
        |   |   |
        +---+---+
        |   | A
       G|   |A
        |   |
        +---+
          F
    */

    // manually inserting all the lines
    use Direction::*;

    // A
    put_coupled_warp_lines(
        &mut field,
        ((150, 51), RIGHT),
        ((151, 50), DOWN),
        50,
        DOWN,
        RIGHT,
        Turn::RIGHT,
    );
    // B
    put_coupled_warp_lines(
        &mut field,
        ((100, 51), UP),
        ((101, 50), LEFT),
        50,
        LEFT,
        UP,
        Turn::LEFT,
    );
    // C
    put_coupled_warp_lines(
        &mut field,
        ((50, 101), RIGHT),
        ((51, 100), DOWN),
        50,
        DOWN,
        RIGHT,
        Turn::RIGHT,
    );
    // D
    put_coupled_warp_lines(
        &mut field,
        ((50, 51), UP),
        ((101, 1), DOWN),
        50,
        LEFT,
        LEFT,
        Turn::AROUND,
    );
    // E
    put_coupled_warp_lines(
        &mut field,
        ((50, 150), UP),
        ((101, 100), DOWN),
        50,
        RIGHT,
        RIGHT,
        Turn::AROUND,
    );
    // F
    put_coupled_warp_lines(
        &mut field,
        ((1, 101), RIGHT),
        ((200, 1), RIGHT),
        50,
        UP,
        DOWN,
        Turn::Nothing,
    );
    // G
    put_coupled_warp_lines(
        &mut field,
        ((1, 51), RIGHT),
        ((151, 1), DOWN),
        50,
        UP,
        LEFT,
        Turn::RIGHT,
    );

    // todo!()
    field
}

pub fn part1(input: &str) -> usize {
    let ((mut pos, field), (segments, turns)) = parse_input(input);
    let field = put_wrapping_warps(field);

    for (segment, turn) in segments.iter().zip(turns.iter()) {
        pos = advance(pos, *segment, &field);
        pos = pos.turned(*turn);
    }
    pos = advance(pos, *segments.last().unwrap(), &field);

    1000 * pos.row + 4 * pos.col + pos.direction.facing()
}

pub fn part2(input: &str) -> usize {
    let ((mut pos, field), (segments, turns)) = parse_input(input);
    let field = put_cube_warps(field);

    //guided(pos, &field)?;

    for (segment, turn) in segments.iter().zip(turns.iter()) {
        pos = advance(pos, *segment, &field);
        pos = pos.turned(*turn);

        // plot
        // plot(Some(pos), &field)?;
    }
    pos = advance(pos, *segments.last().unwrap(), &field);
    // plot(Some(pos), &field)?;

    1000 * pos.row + 4 * pos.col + pos.direction.facing()
}

#[allow(dead_code)]
fn guided(mut pos: Pos, field: &Grid<Tile>) -> () {
    stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    for c in stdin.keys() {
        // plotting
        print!("{}", termion::clear::All);
        for col in 0..field.cols() {
            print!("{}", termion::cursor::Goto(1, col as u16 + 1,),);
            for (row, tile) in field.iter_col(col).enumerate().rev() {
                print!(
                    "{}",
                    if (row, col) == pos.coord() {
                        match pos.direction.turned(Turn::RIGHT) {
                            Direction::RIGHT => "👉",
                            Direction::DOWN => "👇",
                            Direction::LEFT => "👈",
                            Direction::UP => "👆",
                        }
                    } else {
                        match tile {
                            Tile::Extern | Tile::Space => "  ",
                            Tile::Wall => "🧱",
                            Tile::Warp(_) => "🌀",
                        }
                    }
                );
            }
        }

        match c.unwrap() {
            termion::event::Key::Char('w') => pos.direction = Direction::UP.turned(Turn::LEFT),
            termion::event::Key::Char('a') => pos.direction = Direction::LEFT.turned(Turn::LEFT),
            termion::event::Key::Char('d') => pos.direction = Direction::RIGHT.turned(Turn::LEFT),
            termion::event::Key::Char('s') => pos.direction = Direction::DOWN.turned(Turn::LEFT),

            termion::event::Key::Ctrl('q') => return,

            _ => (),
        }
        pos = advance(pos, 1, field);
    }
    unreachable!()
}

#[allow(dead_code)]
fn plot(pos: Option<Pos>, field: &Grid<Tile>) {
    for row in 0..field.rows() {
        for (col, tile) in field.iter_row(row).enumerate() {
            if pos.is_some_and(|pos| (row, col) == (pos.row, pos.col)) {
                match pos.unwrap().direction {
                    Direction::RIGHT => print!("👉"),
                    Direction::DOWN => print!("👇"),
                    Direction::LEFT => print!("👈"),
                    Direction::UP => print!("👆"),
                }
            } else {
                match tile {
                    Tile::Extern | Tile::Space => print!("  "),
                    Tile::Wall => print!("🧱"),
                    Tile::Warp(_) => print!("🌀"),
                }
            }
        }
        println!();
    }
}
