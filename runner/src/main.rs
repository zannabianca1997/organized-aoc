use std::collections::BTreeMap;

use clap::Parser;

use aoc_library::{Day, Solution};

#[derive(Debug, Parser)]
enum Cli {
    /// List all the AoC solution that are present in the library
    List,
}

lazy_static::lazy_static! {
    static ref LIBRARY :BTreeMap<u16, BTreeMap<u8, Day>> = aoc_library::library();
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::List => list(),
    }
}

fn list() -> anyhow::Result<()> {
    for (year, days) in LIBRARY.iter() {
        println!("# Year {year}");
        for (day, Day { part1, part2 }) in days.iter() {
            print!("- Day {day}: ");
            match part1 {
                Some(Solution::Numeric(_)) => print!("part 1 (numerical output)"),
                Some(Solution::Alpha(_)) => print!("part 1 (string output)"),
                None => (),
            }
            if part1.is_some() && part2.is_some() {
                print!(" and ")
            }
            match part2 {
                Some(Solution::Numeric(_)) => print!("part 2 (numerical output)"),
                Some(Solution::Alpha(_)) => print!("part 2 (string output)"),
                None => (),
            }
            if part1.is_none() && part2.is_none() {
                print!("nothing")
            }
            println!()
        }
    }
    Ok(())
}
