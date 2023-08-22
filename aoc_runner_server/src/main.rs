#![feature(error_reporter)]

use std::{
    borrow::Cow,
    error::Report,
    hint::black_box,
    io::{self, stdin, stdout, Write},
    num::NonZeroUsize,
};

use chrono::Duration;
use thiserror::Error;

use aoc::{AoCDay, AoCPart, AoCYear, Solution};
use aoc_index::AOC;
use aoc_runner_binds::*;

pub fn list(year: AoCYear, day: AoCDay, part: AoCPart) -> impl Iterator<Item = SolutionInfo> {
    AOC.get_part(year, day, part)
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
        res = match AOC.get_part(year, day, part)[idx].fun {
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
    (match AOC.get_part(year, day, part)[idx].fun {
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

#[derive(Debug, Error)]
enum ReplError {
    #[error("Error while reading request")]
    In(#[source] io::Error),
    #[error("Error while parsing request")]
    Parse(#[source] serde_json::Error),
    #[error("Error while printing response")]
    Out(#[source] io::Error),
}

fn main() {
    for l in stdin().lines() {
        match l
            .map_err(ReplError::In)
            .and_then(|l| serde_json::from_str(&l).map_err(ReplError::Parse))
            .map(exec)
            .map(|res| serde_json::to_string(&res).unwrap())
            .and_then(|l| writeln!(stdout(), "{l}").map_err(ReplError::Out))
        {
            Ok(()) => (),
            Err(err) => eprintln!("Error: {}", Report::new(err).pretty(true)),
        }
    }
}
