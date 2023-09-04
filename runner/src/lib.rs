use std::{collections::BTreeMap, ffi::OsString, fmt::Debug};

use clap::Parser;

#[derive(Clone, Debug, Copy)]
pub enum Part {
    Numeric(fn(&str) -> i64),
    Alpha(fn(&str) -> String),
    Multiline(fn(&str) -> String),
}

#[derive(Debug)]

pub struct Day([Option<Part>; 2]);
impl Day {
    pub fn add_part_1(&mut self, sol: Part) -> &mut Self {
        self.0[0] = Some(sol);
        self
    }
    pub fn add_part_2(&mut self, sol: Part) -> &mut Self {
        self.0[1] = Some(sol);
        self
    }
}

#[derive(Debug)]
pub struct Year(BTreeMap<u8, Day>);
impl Year {
    pub fn add_day<F>(&mut self, day: u8, build: F) -> &mut Self
    where
        F: FnOnce(&mut Day),
    {
        if day < 1 || day > 25 {
            panic!("Day {day} is invalid, not an advent day")
        }
        (build)(self.0.entry(day).or_insert_with(|| Day([None; 2])));
        self
    }
}

#[derive(Debug)]
pub struct Library(BTreeMap<u16, Year>);

impl Library {
    pub fn add_year<F>(&mut self, year: u16, build: F) -> &mut Self
    where
        F: FnOnce(&mut Year),
    {
        if year < 2015 {
            panic!("Year {year} is invalid, AoC was not present before 2015")
        }
        (build)(self.0.entry(year).or_insert_with(|| Year(BTreeMap::new())));
        self
    }
}

#[derive(Parser)]
struct Args {}

pub fn run<F, I, T>(build: F, args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    F: FnOnce(&mut Library),
{
    let Args {} = Args::parse_from(args);
    // building library
    let library = {
        let mut lib = Library(BTreeMap::new());
        (build)(&mut lib);
        lib
    };
    // executing tests
    todo!()
}
