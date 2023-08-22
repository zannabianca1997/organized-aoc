#![feature(is_some_and)]
#![feature(drain_filter)]
#![feature(box_patterns)]

use std::fmt::Debug;

use calendar::{AoCDay, AoCPart, AoCYear};

pub mod __private;
pub mod calendar;

#[derive(Debug, Clone, Copy)]
pub struct Solution {
    pub name: &'static str,
    pub long_running: bool,
    pub descr: Option<&'static str>,
    pub fun: SolutionFn,
}

#[derive(Clone, Copy)]
pub enum SolutionFn {
    Numeric(fn(&str) -> i64),
    Alpha(fn(&str) -> String),
    Multiline(fn(&str) -> String),
}
impl SolutionFn {
    /// Returns `true` if the solution fn is [`Multiline`].
    ///
    /// [`Multiline`]: SolutionFn::Multiline
    #[must_use]
    pub const fn is_multiline(&self) -> bool {
        matches!(self, Self::Multiline(..))
    }
}

impl Debug for SolutionFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct IgnoredField;
        impl Debug for IgnoredField {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "_")
            }
        }

        match self {
            Self::Numeric(_) => f.debug_tuple("Numeric").field(&IgnoredField).finish(),
            Self::Alpha(_) => f.debug_tuple("Alpha").field(&IgnoredField).finish(),
            Self::Multiline(_) => f.debug_tuple("Multiline").field(&IgnoredField).finish(),
        }
    }
}

/// A library of AoC solutions
pub struct Library(pub [&'static Year; AoCYear::NUM_YEARS]);

impl Library {
    pub fn get_year(&self, year: AoCYear) -> &'static Year {
        self.0[year.idx()]
    }
    pub fn get_day(&self, year: AoCYear, day: AoCDay) -> &'static Day {
        self.get_year(year).get_day(day)
    }
    pub fn get_part(
        &self,
        year: AoCYear,
        day: AoCDay,
        part: AoCPart,
    ) -> &'static [&'static Solution] {
        self.get_year(year).get_day(day).get_part(part)
    }
}

/// A year of AoC solutions
pub struct Year(pub [&'static Day; AoCDay::NUM_DAYS]);

impl Year {
    pub fn get_day(&self, day: AoCDay) -> &'static Day {
        self.0[day.idx()]
    }
    pub fn get_part(&self, day: AoCDay, part: AoCPart) -> &'static [&'static Solution] {
        self.get_day(day).get_part(part)
    }
}

/// A year of AoC solutions
pub struct Day {
    pub part1: &'static [&'static Solution],
    pub part2: &'static [&'static Solution],
}

impl Day {
    pub fn get_part(&self, part: AoCPart) -> &'static [&'static Solution] {
        match part {
            AoCPart::First => self.part1,
            AoCPart::Second => self.part2,
        }
    }
}
