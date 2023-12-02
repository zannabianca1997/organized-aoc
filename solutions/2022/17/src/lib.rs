const ROCKS: &[&[&[bool]]] = &[
    &[&[true, true, true, true]],
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false],
    ],
    &[
        &[true, true, true],
        &[false, false, true],
        &[false, false, true],
    ],
    &[&[true], &[true], &[true], &[true]],
    &[&[true, true], &[true, true]],
];

#[derive(Clone, Copy)]
enum PushDirection {
    LEFT,
    RIGHT,
}

fn parse_input(input: &str) -> Box<[PushDirection]> {
    use PushDirection::*;
    input
        .trim()
        .chars()
        .map(|ch| match ch {
            '<' => LEFT,
            '>' => RIGHT,
            ch => panic!("Unrecognized char {ch}"),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

/// Check if the rock collided
fn collide(rock: &[&[bool]], rock_left: usize, rock_bottom: usize, lines: &[[bool; 7]]) -> bool {
    for (i, rock_line) in rock.iter().enumerate() {
        for (j, &v) in rock_line.iter().enumerate() {
            if v && lines[rock_bottom + i][rock_left + j] {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let pushes = parse_input(input);
    let mut pushes_iter = pushes.iter().cycle();

    let mut lines: Vec<[bool; 7]> = vec![];
    let mut first_empty_line = 0;

    // dropping rocks
    for &rock in ROCKS.iter().cycle().take(2022) {
        let mut rock_bottom = first_empty_line + 3;
        let mut rock_left = 2;
        let rock_height = rock.len();
        let rock_width = rock[0].len();

        // print_state(rock, rock_left, rock_bottom, &lines);

        // adding additional lines as needed
        if lines.len() < rock_bottom + rock_height {
            lines.resize(rock_bottom + rock_height, [false; 7])
        }

        // drop the rock until it rests
        'drop: loop {
            // false push
            match *pushes_iter.next().unwrap() {
                PushDirection::LEFT => {
                    if rock_left != 0 {
                        let new_left = rock_left - 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
                PushDirection::RIGHT => {
                    if rock_left + rock_width != 7 {
                        let new_left = rock_left + 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
            }
            // Drop down
            if rock_bottom == 0 {
                break 'drop;
            }
            let new_bottom = rock_bottom - 1;
            if collide(rock, rock_left, new_bottom, &lines) {
                break 'drop;
            }
            rock_bottom = new_bottom;
        }

        // Rock has come to a rest. adding it to the lines...
        for (i, line) in rock.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                lines[rock_bottom + i][rock_left + j] |= v;
            }
        }
        // measuring heigth of the tower...
        first_empty_line = lines.len();
        while lines[first_empty_line - 1] == [false; 7] {
            first_empty_line -= 1;
        }
    }

    // measuring heigth of the tower...
    first_empty_line
}

#[allow(dead_code)]
fn print_state(rock: &[&[bool]], rock_left: usize, rock_bottom: usize, lines: &[[bool; 7]]) {
    for h in (0..lines.len()).rev() {
        print!("|");
        for x in 0..7 {
            if rock_bottom <= h
                && h < rock_bottom + rock.len()
                && rock_left <= x
                && x < rock_left + rock[h - rock_bottom].len()
                && rock[h - rock_bottom][x - rock_left]
            {
                print!("@");
            }
            if lines[h][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!();
}

pub fn part2(input: &str) -> usize {
    let pushes = parse_input(input);
    let mut pushes_iter = pushes.iter().enumerate().cycle().peekable();

    let mut lines: Vec<[bool; 7]> = vec![];
    let mut first_empty_line = 0;

    let mut jump_happened = false;
    let mut additional_height = 0;
    let mut additional_rocks = 0;

    let mut log = vec![];
    let mut last_height = 0;

    // dropping rocks
    for (rock_total, (rock_num, &rock)) in ROCKS.iter().enumerate().cycle().enumerate() {
        if rock_total + additional_rocks == 1000000000000 {
            break;
        }
        let mut rock_bottom = first_empty_line + 3;
        let mut rock_left = 2;
        let rock_height = rock.len();
        let rock_width = rock[0].len();

        if !jump_happened && rock_num == 0 {
            // logging heigth diffs and push cycle points
            let heigth_diff = first_empty_line - last_height;
            last_height = first_empty_line;
            log.push((pushes_iter.peek().unwrap().0, heigth_diff));
            // searching for a repetition in the log
            for rep_len in (1..log.len() / 2).rev() {
                if log[log.len() - 2 * rep_len..log.len() - rep_len] == log[log.len() - rep_len..] {
                    // we have a loop
                    // TODO: check the tiling?
                    // measure the loop
                    let rep_rocks = ROCKS.len() * rep_len;
                    let rep_height: usize =
                        log[log.len() - rep_len..].iter().map(|(_, dh)| *dh).sum();
                    // calculate number of repetitions
                    let repeats = (1000000000000 - rock_total) / rep_rocks;

                    additional_rocks = repeats * rep_rocks;
                    additional_height = repeats * rep_height;
                    jump_happened = true;

                    // dbg!(repeats, additional_rocks, additional_height);
                }
            }
        }

        // adding additional lines as needed
        if lines.len() < rock_bottom + rock_height {
            lines.resize(rock_bottom + rock_height, [false; 7])
        }

        // drop the rock until it rests
        'drop: while let Some((_, &push)) = pushes_iter.next() {
            // lateral push
            match push {
                PushDirection::LEFT => {
                    if rock_left != 0 {
                        let new_left = rock_left - 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
                PushDirection::RIGHT => {
                    if rock_left + rock_width != 7 {
                        let new_left = rock_left + 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
            }
            // Drop down
            if rock_bottom == 0 {
                break 'drop;
            }
            let new_bottom = rock_bottom - 1;
            if collide(rock, rock_left, new_bottom, &lines) {
                break 'drop;
            }
            rock_bottom = new_bottom;
        }

        // Rock has come to a rest. adding it to the lines...
        for (i, line) in rock.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                lines[rock_bottom + i][rock_left + j] |= v;
            }
        }

        // measuring heigth of the tower...
        first_empty_line = lines.len();
        while lines.get(first_empty_line - 1) == Some(&[false; 7]) {
            first_empty_line -= 1;
        }
    }

    // measuring heigth of the tower...
    first_empty_line + additional_height
}
