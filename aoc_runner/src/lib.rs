use std::{borrow::Cow, hint::black_box, num::NonZeroUsize};

use chrono::Duration;

use aoc::{AoCDay, AoCPart, AoCYear, Solution};
use aoc_runner_binds::*;
use index::LIBRARY;

pub fn list(year: AoCYear, day: AoCDay, part: AoCPart) -> impl Iterator<Item = SolutionInfo> {
    LIBRARY
        .get_part(year, day, part)
        .into_iter()
        .copied()
        .copied()
        .enumerate()
        .map(
            move |(
                idx,
                Solution {
                    name,
                    long_running,
                    descr,
                    fun,
                },
            )| {
                SolutionInfo {
                    year,
                    day,
                    part,
                    idx,
                    multiline: fun.is_multiline(),
                    long_running,
                    name: Cow::Borrowed(name),
                    descr: descr.map(Cow::Borrowed),
                }
            },
        )
}

pub fn run(
    year: AoCYear,
    day: AoCDay,
    part: AoCPart,
    idx: usize,
    input: &str,
) -> (String, Duration) {
    let mut res = Default::default();
    let time = Duration::span(|| {
        res = match LIBRARY.get_part(year, day, part)[idx].fun {
            aoc::SolutionFn::Numeric(fun) => fun(input).to_string(),
            aoc::SolutionFn::Alpha(fun) | aoc::SolutionFn::Multiline(fun) => fun(input),
        }
    });
    (res, time)
}

pub fn timeit(
    year: AoCYear,
    day: AoCDay,
    part: AoCPart,
    idx: usize,
    input: &str,
    reps: NonZeroUsize,
) -> Duration {
    (match LIBRARY.get_part(year, day, part)[idx].fun {
        aoc::SolutionFn::Numeric(fun) => Duration::span(|| {
            for _ in 0..reps.get() {
                black_box(fun(black_box(input)));
            }
        }),
        aoc::SolutionFn::Alpha(fun) | aoc::SolutionFn::Multiline(fun) => Duration::span(|| {
            for _ in 0..reps.get() {
                black_box(fun(black_box(input)));
            }
        }),
    } / reps.get() as i32)
}

pub fn exec(req: Request) -> Response {
    match req {
        Request::List { year, day, part } => Response::List {
            found: list(year, day, part).collect(),
        },
        Request::Run {
            idx,
            input,
            year,
            day,
            part,
        } => {
            let (answer, time) = run(year, day, part, idx, &input);
            Response::Run { answer, time }
        }
        Request::TimeIt {
            year,
            day,
            part,
            idx,
            reps,
            input,
        } => Response::TimeIt {
            time: timeit(year, day, part, idx, &input, reps),
        },
    }
}
