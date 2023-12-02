#[derive(Debug, Clone, Copy)]
struct SectionRange(i64, i64);
impl SectionRange {
    fn is_inside(self, other: SectionRange) -> bool {
        other.0 <= self.0 && self.1 <= other.1
    }
    fn overlap(self, other: SectionRange) -> bool {
        (other.0 <= self.0 && self.0 <= other.1)
            || (other.0 <= self.1 && self.1 <= other.1)
            || other.is_inside(self)
    }
}
impl From<&str> for SectionRange {
    fn from(value: &str) -> Self {
        let (s1, s2) = value.split_once("-").unwrap();
        let begin = s1.trim().parse().unwrap();
        let end = s2.trim().parse().unwrap();
        Self(begin, end)
    }
}

fn read_input(input: &str) -> Vec<(SectionRange, SectionRange)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once(",").unwrap();
            (SectionRange::from(s1), SectionRange::from(s2))
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    read_input(input)
        .into_iter()
        .filter(|(r1, r2)| r1.is_inside(*r2) || r2.is_inside(*r1))
        .count()
}

pub fn part2(input: &str) -> usize {
    read_input(input)
        .into_iter()
        .filter(|(r1, r2)| r1.overlap(*r2))
        .count()
}
