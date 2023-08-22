//! # Build aoc libraries
//!
//!
//! ```
//! use aoc::{library, year, day, solution, Library, Year, Day};
//!
//! static LIBRARY: Library = library! {
//!     2015 => AOC_2015,
//! };
//!
//!
//! static AOC_2015: Year = year! {
//!     1 => AOC_2015_1,
//! };
//!
//! static AOC_2015_1: Day = day! {
//!     part1, // equal to `part1: part1` or `part1: [part1]`
//!     part2: [part2_multiline, part2_long]
//! };
//!
//!/*
//! #[solution]
//! fn part1(input: &str) -> i64 {
//!     todo!()
//! }
//!
//! #[solution(long_running, descr= "Added parsing, but is slower")]
//! fn part2_long(input: &str) -> i64 {
//!     todo!()
//! }
//!
//! #[solution(multiline)]
//! fn part2_multiline(input: &str) -> String {
//!     todo!()
//! }
//!*/
//! ```

pub use aoc_macros::*;
pub use aoc_runtime::{Day, Library, Year, __private};
