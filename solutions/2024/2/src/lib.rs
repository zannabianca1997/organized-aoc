#![feature(iter_map_windows)]
#![feature(iter_collect_into)]

fn lists(input: &str) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_ {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()))
}

#[derive(Debug, Clone, Copy)]
struct SafeState {
    failed: bool,
    last: usize,
    direction: std::cmp::Ordering,
}
impl SafeState {
    fn new(v1: usize, v2: usize) -> Self {
        let dist = usize::abs_diff(v1, v2);
        Self {
            failed: dist < 1 || dist > 3,
            last: v2,
            direction: usize::cmp(&v1, &v2),
        }
    }

    fn accept(self, v: usize) -> Self {
        let Self {
            last,
            direction,
            failed,
        } = self;
        let dist = usize::abs_diff(last, v);
        Self {
            last: v,
            direction,
            failed: failed || dist < 1 || dist > 3 || direction != usize::cmp(&last, &v),
        }
    }
}

fn safe(mut line: impl Iterator<Item = usize>) -> bool {
    let Some(v1) = line.next() else {
        return true;
    };
    let Some(v2) = line.next() else {
        return true;
    };
    let mut state = SafeState::new(v1, v2);
    for v in line {
        if state.failed {
            return false;
        }
        state = state.accept(v)
    }
    !state.failed
}

pub fn part1(input: &str) -> usize {
    lists(input).filter_map(|l| safe(l).then_some(())).count()
}

pub fn part2(input: &str) -> usize {
    let mut l = vec![];
    lists(input)
        .filter_map(|line| {
            l.clear();
            line.collect_into(&mut l);
            safe(l.iter().copied()).then_some(()).or_else(|| {
                for i in 0..l.len() {
                    if safe(
                        l.iter()
                            .copied()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, c)| c),
                    ) {
                        return Some(());
                    }
                }
                None
            })
        })
        .count()
}
