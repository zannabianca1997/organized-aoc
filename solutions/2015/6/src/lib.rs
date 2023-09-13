use rects::Rect;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    command: Command,
    rect: Rect<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    On,
    Off,
    Toggle,
}

fn parse(input: &str) -> impl DoubleEndedIterator<Item = Instruction> + '_ {
    input.lines().filter(|s| !s.trim().is_empty()).map(|l| {
        let (command, p1) = match l.as_bytes()[6] {
            b'n' => (Command::On, 2),
            b'f' => (Command::Off, 2),
            b' ' => (Command::Toggle, 1),
            _ => panic!(),
        };
        let mut words = l.split_ascii_whitespace();
        let (minx, miny) = words.nth(p1).unwrap().split_once(',').unwrap();
        let (maxx, maxy) = words.nth(1).unwrap().split_once(',').unwrap();
        Instruction {
            command,
            rect: Rect {
                minx: minx.parse().unwrap(),
                maxx: maxx.parse::<usize>().unwrap() + 1,
                miny: miny.parse().unwrap(),
                maxy: maxy.parse::<usize>().unwrap() + 1,
            },
        }
    })
}

pub fn part1(input: &str) -> usize {
    // remaining rects
    let mut untoggled = vec![Rect {
        minx: 0,
        maxx: 1000,
        miny: 0,
        maxy: 1000,
    }];
    let mut toggled = vec![];
    // count of cell found
    let mut count = 0usize;

    for Instruction {
        command,
        rect: affected_rect,
    } in parse(input).rev()
    {
        let mut new_untoggled = vec![];
        let mut new_toggled = vec![];

        match command {
            Command::On => {
                for rect in untoggled {
                    let (outside, inside) = affected_rect.cut(rect);
                    new_untoggled.extend_from_slice(&outside);
                    count += inside.map_or(0, |r| r.area())
                }
                for rect in toggled {
                    let (outside, _) = affected_rect.cut(rect);
                    new_toggled.extend_from_slice(&outside);
                }
            }
            Command::Off => {
                for rect in untoggled {
                    let (outside, _) = affected_rect.cut(rect);
                    new_untoggled.extend_from_slice(&outside);
                }
                for rect in toggled {
                    let (outside, inside) = affected_rect.cut(rect);
                    new_toggled.extend_from_slice(&outside);
                    count += inside.map_or(0, |r| r.area())
                }
            }
            Command::Toggle => {
                for rect in untoggled {
                    let (outside, inside) = affected_rect.cut(rect);
                    new_untoggled.extend_from_slice(&outside);
                    new_toggled.extend(inside)
                }
                for rect in toggled {
                    let (outside, inside) = affected_rect.cut(rect);
                    new_toggled.extend_from_slice(&outside);
                    new_untoggled.extend(inside)
                }
            }
        }

        untoggled = new_untoggled;
        toggled = new_toggled;
    }
    count + toggled.into_iter().map(|r| r.area()).sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let mut rects = vec![(
        0usize,
        Rect {
            minx: 0,
            maxx: 1000,
            miny: 0,
            maxy: 1000,
        },
    )];
    for Instruction {
        command,
        rect: affected_rect,
    } in parse(input)
    {
        let mut new_rects = vec![];
        let delta = match command {
            Command::On => 1,
            Command::Off => -1,
            Command::Toggle => 2,
        };
        for (state, rect) in rects {
            let (outside, inside) = affected_rect.cut(rect);
            // unaffected parts
            new_rects.extend(outside.into_iter().map(|r| (state, r)));
            if let Some(inside) = inside {
                // toggle
                new_rects.push((state.saturating_add_signed(delta), inside))
            }
        }
        rects = new_rects;
    }
    rects.into_iter().map(|(s, r)| s * r.area()).sum()
}
