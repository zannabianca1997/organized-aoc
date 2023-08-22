#![feature(error_reporter)]

use std::{
    error::Report,
    io::{self, stdin, stdout, Write},
};

use aoc_runner::exec;
use thiserror::Error;

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
