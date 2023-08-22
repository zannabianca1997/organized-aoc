use std::fmt::Debug;

use calendar::{AoCDay, AoCPart, AoCYear};

pub mod __private;
pub mod calendar;

#[derive(Debug, Clone, Copy)]
pub struct Solution {
    pub long_running: bool,
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
pub struct Library(pub [Option<&'static Year>; AoCYear::NUM_YEARS]);

/// A year of AoC solutions
pub struct Year(pub [Option<&'static Day>; AoCDay::NUM_DAYS]);
/// A year of AoC solutions
pub struct Day {
    pub part1: &'static [&'static Solution],
    pub part2: &'static [&'static Solution],
}
