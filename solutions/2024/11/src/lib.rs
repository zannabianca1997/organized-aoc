use std::{collections::HashSet, hash::BuildHasher};

use fnv::FnvHashMap;

pub fn parse(input: &str) -> FnvHashMap<usize, usize> {
    let mut stones = FnvHashMap::default();
    for stone in input.split_whitespace() {
        *stones.entry(stone.parse().unwrap()).or_insert(0) += 1
    }
    stones
}

pub fn part1(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..25 {
        let mut new_stones = FnvHashMap::default();
        for (number_written, count) in stones {
            if number_written == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if digits(number_written) % 2 == 0 {
                let (a, b) = split(number_written);
                *new_stones.entry(a).or_insert(0) += count;
                *new_stones.entry(b).or_insert(0) += count;
            } else {
                *new_stones.entry(number_written * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones
    }
    stones.values().sum()
}
pub fn part2(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..75 {
        let mut new_stones = FnvHashMap::default();
        for (number_written, count) in stones {
            if number_written == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if digits(number_written) % 2 == 0 {
                let (a, b) = split(number_written);
                *new_stones.entry(a).or_insert(0) += count;
                *new_stones.entry(b).or_insert(0) += count;
            } else {
                *new_stones.entry(number_written * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones
    }
    stones.values().sum()
}

#[inline(always)]
fn split(number_written: usize) -> (usize, usize) {
    let split_at = digits(number_written) / 2;
    let tens = 10usize.pow(split_at as _);
    let right = number_written % tens;
    ((number_written - right) / tens, right)
}

#[cfg(test)]
mod tests {
    mod digits {
        use crate::digits;

        #[test]
        fn _0() {
            assert_eq!(digits(0), 1)
        }
        #[test]
        fn _5() {
            assert_eq!(digits(5), 1)
        }
        #[test]
        fn _10() {
            assert_eq!(digits(10), 2)
        }
        #[test]
        fn _234() {
            assert_eq!(digits(234), 3)
        }
    }
    mod split {
        use crate::split;

        #[test]
        fn _23() {
            assert_eq!(split(23), (2, 3))
        }

        #[test]
        fn _1563() {
            assert_eq!(split(1563), (15, 63))
        }
    }
}

#[inline(always)]
fn digits(a: usize) -> usize {
    if a == 0 {
        return 1;
    }
    let mut ten = 1;
    let mut digits = 0;
    while ten <= a {
        ten *= 10;
        digits += 1;
    }
    digits
}
