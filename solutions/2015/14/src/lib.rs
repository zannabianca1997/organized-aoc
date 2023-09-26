#![feature(iter_next_chunk)]

use std::num::NonZeroUsize;

struct Reindeer {
    speed: NonZeroUsize,
    time: NonZeroUsize,
    rest: NonZeroUsize,
}

fn parse(input: &str) -> impl Iterator<Item = Reindeer> + '_ {
    // Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
    input.trim().lines().map(|l| {
        let [_, _, _, s, _, _, t, _, _, _, _, _, _, r, _] = l.splitn(15, ' ').next_chunk().unwrap();
        Reindeer {
            speed: s.parse().unwrap(),
            time: t.parse().unwrap(),
            rest: r.parse().unwrap(),
        }
    })
}

struct Runner {
    data: Reindeer,
    state: RunnerState,
    pos: usize,
}
impl Runner {
    fn new(data: Reindeer) -> Self {
        Self {
            state: RunnerState::Running(data.time),
            pos: 0,
            data,
        }
    }

    fn run<const TIME: usize>(&mut self) {
        let mut time = TIME;
        while time >= self.state.remaining_time().get() {
            match self.state {
                RunnerState::Running(t) => {
                    let t = t.get();
                    time -= t;
                    self.pos += self.data.speed.get() * t;
                    self.state = RunnerState::Resting(self.data.rest)
                }
                RunnerState::Resting(t) => {
                    let t = t.get();
                    time -= t;
                    self.state = RunnerState::Running(self.data.time)
                }
            }
        }
        match &mut self.state {
            RunnerState::Running(t) => {
                self.pos += self.data.speed.get() * time;
                *t = NonZeroUsize::new(t.get() - time).unwrap()
            }
            RunnerState::Resting(t) => *t = NonZeroUsize::new(t.get() - time).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RunnerState {
    Running(NonZeroUsize),
    Resting(NonZeroUsize),
}
impl RunnerState {
    fn remaining_time(&mut self) -> &mut NonZeroUsize {
        match self {
            RunnerState::Running(t) | RunnerState::Resting(t) => t,
        }
    }
}

const RACE_LEN: usize = 2503;

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|r| {
            let mut runner = Runner::new(r);
            runner.run::<RACE_LEN>();
            runner.pos
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut runners: Vec<(Runner, usize)> = parse(input).map(|r| (Runner::new(r), 0)).collect();
    for _ in 0..RACE_LEN {
        for (r, _) in runners.iter_mut() {
            r.run::<1>()
        }
        let lead = runners.iter().map(|(r, _)| r.pos).max().unwrap();
        for (r, pts) in runners.iter_mut() {
            if r.pos == lead {
                *pts += 1
            }
        }
    }
    runners.into_iter().map(|(_, p)| p).max().unwrap()
}
