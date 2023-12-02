#[derive(Debug, Clone, Copy)]
struct Move {
    quantity: u8,
    from: u8,
    to: u8,
}

fn parse_drawing(input: &str) -> Vec<Vec<char>> {
    // splitting lines
    let (header, stacks_lines) = {
        let mut lines = input.lines().rev().filter(|l| l.trim() != "");
        let header = lines.next().unwrap();
        (header, lines.collect::<Vec<_>>())
    };
    // counting the number and position of the rows
    let col_pos: Vec<_> = header
        .chars()
        .enumerate()
        .filter(|&(_, ch)| '1' <= ch && ch <= '9')
        .map(|(pos, ch)| (pos, ch.to_digit(10).unwrap() - 1))
        .collect();
    // making the stacks
    let mut stacks: Vec<Vec<char>> = (0..col_pos.len())
        .map(|_| Vec::with_capacity(stacks_lines.len()))
        .collect();
    // filling them
    for stack_line in stacks_lines {
        for (pos, col) in col_pos.iter() {
            if let Some(ch) = stack_line.chars().nth(*pos) {
                if ch != ' ' {
                    stacks[*col as usize].push(ch)
                }
            }
        }
    }

    stacks
}
fn parse_moves(input: &str) -> Vec<Move> {
    /*
        // This is a cleaner REGEX solution. Sadly, also a lot slower

        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        }
        input.trim().lines().map(|line| {
            let m = RE.captures(line).unwrap()
            Move {
                quantity: m.get(1).unwrap().as_str().parse().unwrap(),
                from: m.get(2).unwrap().as_str().parse::<u8>().unwrap() - 1,
                to: m.get(3).unwrap().as_str().parse::<u8>().unwrap() - 1,
            }
        }).collect()
    */
    input
        .trim()
        .lines()
        .map(|line| {
            let (qt, mov) = line
                .trim()
                // removing move
                .strip_prefix("move")
                .unwrap()
                // splitting parts
                .split_once("from")
                .unwrap();
            let (from, to) = mov.split_once("to").unwrap();
            // converting into ints
            let qt = qt.trim().parse::<u8>().unwrap();
            let from = from.trim().parse::<u8>().unwrap() - 1;
            let to = to.trim().parse::<u8>().unwrap() - 1;
            // mapping to proprietary type
            Move {
                quantity: qt,
                from,
                to,
            }
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    // splitting the drawing from the move set
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let drawing = parse_drawing(drawing);
    let moves = parse_moves(moves);
    (drawing, moves)
}

fn stack_tops(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|stack| stack.last().map(|ch| *ch).unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for mov in moves {
        for _ in 0..mov.quantity {
            let item = stacks[mov.from as usize].pop().unwrap();
            stacks[mov.to as usize].push(item)
        }
    }
    stack_tops(stacks)
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for mov in moves {
        let mut items = vec![];
        for _ in 0..mov.quantity {
            items.push(stacks[mov.from as usize].pop().unwrap())
        }
        while let Some(item) = items.pop() {
            stacks[mov.to as usize].push(item)
        }
    }
    stack_tops(stacks)
}
