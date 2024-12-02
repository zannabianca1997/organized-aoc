#![feature(array_windows)]
use grid::Grid;

fn parse_input(input: &str) -> Vec<Vec<(isize, isize)>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split("->")
                .map(|pos| {
                    let (p1, p2) = pos.split_once(',').unwrap();
                    (p1.trim().parse().unwrap(), p2.trim().parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn make_field(
    input: Vec<Vec<(isize, isize)>>,
    drop_pos: (isize, isize),
    floor: bool,
) -> (Grid<bool>, (isize, isize)) {
    let (min_x, max_x, min_y, max_y) = input.iter().flat_map(|line| line.iter()).fold(
        (drop_pos.0, drop_pos.0, drop_pos.1, drop_pos.1),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    let (min_x, max_x, min_y, max_y) = if floor {
        let floor_y = max_y + 2;
        let drop = floor_y - drop_pos.1;
        assert!(drop > 0);
        // open enough space to the sides for the mound
        let min_x = min_x.min(drop_pos.0 - drop);
        let max_x = max_x.max(drop_pos.0 + drop);
        // open space for the floor
        let max_y = max_y + 1;
        (min_x, max_x, min_y, max_y)
    } else {
        // open a little space to let it drop at the side
        let min_x = min_x - 1;
        let max_x = max_x + 1;
        (min_x, max_x, min_y, max_y)
    };
    // move drop pos to the relative position
    let drop_pos = (drop_pos.0 - min_x, drop_pos.1 - min_y);

    let mut field = Grid::new(
        (max_x + 1 - min_x) as usize,
        (max_y + 1 - min_y) as usize,
        false,
    );
    dbg!(field.shape());
    for line in input {
        for [(x1, y1), (x2, y2)] in line.array_windows().copied() {
            if y1 == y2 {
                let y = y1 - min_y;
                for x in x1.min(x2)..=x1.max(x2) {
                    let x = x - min_x;
                    field[(x as usize, y as usize)] = true;
                }
            } else if x1 == x2 {
                let x = x1 - min_x;
                for y in y1.min(y2)..=y1.max(y2) {
                    let y = y - min_y;
                    field[(x as usize, y as usize)] = true;
                }
            } else {
                panic!("Line is not orthogonal");
            }
        }
    }
    (field, drop_pos)
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let drop_pos: (isize, isize) = (500, 0);
    let (mut field, drop_pos) = make_field(input, drop_pos, false);

    let mut deposited_grains = 0;
    'grains: loop {
        // drop a grain
        let mut pos = drop_pos;
        if field[(pos.0 as usize, pos.1 as usize)] == true {
            panic!("Sand filled to the drop start");
        }
        'fall: loop {
            match (
                if pos.0 > 0 {
                    field.get((pos.0 - 1) as usize, (pos.1 + 1) as usize)
                } else {
                    None
                },
                field.get(pos.0 as usize, (pos.1 + 1) as usize),
                if pos.0 > 0 {
                    field.get((pos.0 + 1) as usize, (pos.1 + 1) as usize)
                } else {
                    None
                },
            ) {
                (None, None, None) => break 'grains, // reached the bottom, fall infinitely

                (_, Some(false), _) => pos.1 += 1, // drop down
                (Some(false), Some(true), _) => {
                    pos.1 += 1;
                    pos.0 -= 1
                } // drop right
                (Some(true) | None, Some(true), Some(false)) => {
                    pos.1 += 1;
                    pos.0 += 1
                } // drop left

                (Some(true) | None, Some(true), Some(true) | None) => {
                    deposited_grains += 1;
                    field[(pos.0 as usize, pos.1 as usize)] = true;
                    break 'fall;
                } // rest

                _ => unreachable!(),
            }
        }
    }

    deposited_grains
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let drop_pos: (isize, isize) = (500, 0);
    let (mut field, drop_pos) = make_field(input, drop_pos, true);

    let mut deposited_grains = 0;
    'grains: loop {
        // drop a grain
        let mut pos = drop_pos;
        if field[(pos.0 as usize, pos.1 as usize)] == true {
            break 'grains;
        }
        'fall: loop {
            match (
                if pos.0 > 0 {
                    field.get((pos.0 - 1) as usize, (pos.1 + 1) as usize)
                } else {
                    None
                },
                field.get(pos.0 as usize, (pos.1 + 1) as usize),
                if pos.0 > 0 {
                    field.get((pos.0 + 1) as usize, (pos.1 + 1) as usize)
                } else {
                    None
                },
            ) {
                (_, Some(false), _) => pos.1 += 1, // drop down
                (Some(false), Some(true), _) => {
                    pos.1 += 1;
                    pos.0 -= 1
                } // drop right
                (Some(true) | None, Some(true), Some(false)) => {
                    pos.1 += 1;
                    pos.0 += 1
                } // drop left

                (Some(true) | None, Some(true) | None, Some(true) | None) => {
                    deposited_grains += 1;
                    field[(pos.0 as usize, pos.1 as usize)] = true;
                    break 'fall;
                } // rest

                _ => unreachable!(),
            }
        }
    }

    deposited_grains
}
