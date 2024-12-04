use grid::Grid;

pub fn parse(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().filter(|l| !l.trim().is_empty()).count();
    let mut grid = Grid::new(height, width, ' ');
    for (y, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(y, x)] = ch;
        }
    }
    grid
}

pub fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut counter = 0;

    for x in 0..input.shape().0 {
        for y in 0..input.shape().1 {
            if input[(x, y)] == 'X' {
                // possible start point
                for (dx, dy) in [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ] {
                    if [
                        input.get(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)),
                        input.get(x.wrapping_add_signed(2 * dx), y.wrapping_add_signed(2 * dy)),
                        input.get(x.wrapping_add_signed(3 * dx), y.wrapping_add_signed(3 * dy)),
                    ] == [Some(&'M'), Some(&'A'), Some(&'S')]
                    {
                        counter += 1
                    }
                }
            }
        }
    }

    counter
}

pub fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut counter = 0;

    for x in 0..input.shape().0 {
        for y in 0..input.shape().1 {
            if input[(x, y)] == 'A' {
                // possible start point
                if [(Some(&'M'), Some(&'S')), (Some(&'S'), Some(&'M'))].contains(&(
                    input.get(x.wrapping_add_signed(-1), y.wrapping_add_signed(-1)),
                    input.get(x.wrapping_add_signed(1), y.wrapping_add_signed(1)),
                )) && [(Some(&'M'), Some(&'S')), (Some(&'S'), Some(&'M'))].contains(&(
                    input.get(x.wrapping_add_signed(-1), y.wrapping_add_signed(1)),
                    input.get(x.wrapping_add_signed(1), y.wrapping_add_signed(-1)),
                )) {
                    counter += 1
                }
            }
        }
    }

    counter
}
