#![feature(array_windows)]

fn is_nice(s: &str) -> bool {
    s.bytes().filter(|c| b"aeiou".contains(c)).count() >= 3
        && s.as_bytes().array_windows().any(|[a, b]| a == b)
        && s.as_bytes()
            .array_windows()
            .all(|ab| ![b"ab", b"cd", b"pq", b"xy"].contains(&ab))
}
fn is_nicer(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.array_windows().any(|[a, _, b]| a == b)
        && (|| {
            for i in 0..bytes.len() - 3 {
                for j in i + 2..bytes.len() - 1 {
                    if bytes[i..i + 2] == bytes[j..j + 2] {
                        return true;
                    }
                }
            }
            false
        })()
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l)).count()
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_nicer(l)).count()
}

#[cfg(test)]
mod examples {
    use crate::{is_nice, is_nicer};
    #[test]
    fn ugknbfddgicrmopn() {
        assert!(is_nice("ugknbfddgicrmopn"))
    }
    #[test]
    fn aaa() {
        assert!(is_nice("aaa"))
    }
    #[test]
    fn jchzalrnumimnmhp() {
        assert!(!is_nice("jchzalrnumimnmhp"))
    }
    #[test]
    fn haegwjzuvuyypxyu() {
        assert!(!is_nice("haegwjzuvuyypxyu"))
    }
    #[test]
    fn dvszwmarrgswjxmb() {
        assert!(!is_nice("dvszwmarrgswjxmb"))
    }
    #[test]
    fn qjhvhtzxzqqjkmpb() {
        assert!(is_nicer("qjhvhtzxzqqjkmpb"))
    }
    #[test]
    fn xxyxx() {
        assert!(is_nicer("xxyxx"))
    }
    #[test]
    fn uurcxstgmygtbstg() {
        assert!(!is_nicer("uurcxstgmygtbstg"))
    }
    #[test]
    fn ieodomkazucvgmuy() {
        assert!(!is_nicer("ieodomkazucvgmuy"))
    }
}
