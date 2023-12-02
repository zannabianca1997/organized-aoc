use grid::Grid;

fn parse_input(input: &str) -> Grid<u8> {
    let lines: Vec<_> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = Grid::new(height, width, 0);
    for (y, line) in lines.into_iter().rev().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch.to_digit(10).expect("not a valid height!") as u8;
        }
    }

    grid
}

pub fn part1(input: &str) -> usize {
    let heights = parse_input(input);
    let (max_height, max_width) = heights.shape();
    let mut visible = Grid::new_like(&heights, false);

    for x in 0..max_width {
        // top -> down
        let mut min_h = heights[(x, 0)];
        visible[(x, 0)] = true;
        for y in 1..max_height {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
        // bottom -> up
        let mut min_h = heights[(x, max_height - 1)];
        visible[(x, max_height - 1)] = true;
        for y in (0..(max_height - 1)).rev() {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
    }
    for y in 0..max_height {
        // left -> right
        let mut min_h = heights[(0, y)];
        visible[(0, y)] = true;
        for x in 1..max_width {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
        // right -> left
        let mut min_h = heights[(max_width - 1, y)];
        visible[(max_width - 1, y)] = true;
        for x in (0..(max_width - 1)).rev() {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
    }

    visible.iter().filter(|v| **v).count()
}

pub fn part2(input: &str) -> usize {
    let heights = parse_input(input);
    let (max_heigth, max_width) = heights.shape();

    let mut max_scenic_score = 0;
    for tx in 0..max_width {
        for ty in 0..max_heigth {
            let treehouse_height = heights[(tx, ty)];
            let scenic_score: usize =
                if tx == 0 || ty == 0 || tx == max_width - 1 || ty == max_heigth - 1 {
                    0
                } else {
                    (
                        // up
                        {
                        let mut ray_len = 0;
                        for y in (ty + 1)..max_heigth {
                            ray_len += 1;
                            if heights[(tx, y)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // down
                    {
                        let mut ray_len = 0;
                        for y in (0..=(ty - 1)).rev() {
                            ray_len += 1;
                            if heights[(tx, y)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // left
                    {
                        let mut ray_len = 0;
                        for x in (tx + 1)..max_width {
                            ray_len += 1;
                            if heights[(x, ty)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // right
                    {
                        let mut ray_len = 0;
                        for x in (0..=(tx - 1)).rev() {
                            ray_len += 1;
                            if heights[(x, ty)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    }
                    )
                };

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }
    max_scenic_score
}
