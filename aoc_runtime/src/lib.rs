use std::fmt::Debug;

use calendar::{AoCDay, AoCPart, AoCYear};
pub use linkme::distributed_slice;

pub mod calendar;

#[derive(Debug, Clone, Copy)]
pub struct Solution {
    pub year: AoCYear,
    pub day: AoCDay,
    pub part: AoCPart,
    pub long_running: bool,
    pub fun: SolutionFn,
}

#[derive(Clone, Copy)]
pub enum SolutionFn {
    Numeric(fn(&str) -> i64),
    Alpha(fn(&str) -> String),
    Multiline(fn(&str) -> String),
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

#[distributed_slice]
pub static SOLUTIONS: [Solution] = [..];
