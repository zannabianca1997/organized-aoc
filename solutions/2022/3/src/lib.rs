#![feature(never_type)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Container(u64);
impl Container {
    fn add(self, item: char) -> Option<Self> {
        let priority = if 'a' <= item && item <= 'z' {
            Some(item as u8 - 'a' as u8 + 1)
        } else if 'A' <= item && item <= 'Z' {
            Some(item as u8 - 'A' as u8 + 27)
        } else {
            None
        };
        priority.map(|p| Self(self.0 | 1 << p))
    }
    fn commons(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    fn empty(self) -> bool {
        self.0 == 0
    }
    fn first_priority(self) -> u8 {
        let mut p = 0;
        let mut v = self.0;
        while v % 2 == 0 {
            v >>= 1;
            p += 1;
        }
        p
    }
}
impl From<&str> for Container {
    fn from(value: &str) -> Self {
        let mut res = Self(0);
        for ch in value.chars() {
            res = res.add(ch).unwrap();
        }
        res
    }
}

fn read_input_compartments(input: &str) -> Vec<(Container, Container)> {
    input
        .split("\n")
        .filter_map(|line| {
            let line = line.trim();
            if line == "" {
                None
            } else if line.chars().count() % 2 != 0 {
                panic!("Odd lenght line")
            } else {
                Some({
                    let (s1, s2) = line.split_at(line.chars().count() / 2);
                    (Container::from(s1), Container::from(s2))
                })
            }
        })
        .collect()
}
fn read_input_groups(input: &str) -> Vec<(Container, Container, Container)> {
    let lines: Vec<_> = input
        .split("\n")
        .filter_map(|line| {
            let line = line.trim();
            if line == "" {
                None
            } else {
                Some(Container::from(line))
            }
        })
        .collect();
    if lines.len() % 3 != 0 {
        panic!("Line is not a multiple of three")
    }
    let groups = lines.len() / 3;
    let mut res = Vec::with_capacity(groups);
    let mut lines = lines.into_iter();
    for _ in 0..groups {
        res.push((
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ))
    }
    assert!(lines.next() == None);
    res
}

pub fn part1(input: &str) -> i64 {
    let input = read_input_compartments(input);
    let mut total = 0;
    for (c1, c2) in input {
        let common = Container::commons(c1, c2);
        if !common.empty() {
            total += common.first_priority() as i64;
        } else {
            panic!("No common elements")
        }
    }
    total
}

pub fn part2(input: &str) -> i64 {
    let input = read_input_groups(input);
    let mut total = 0;
    for (c1, c2, c3) in input {
        let common = Container::commons(c1, c2).commons(c3);
        if !common.empty() {
            total += common.first_priority() as i64;
        } else {
            panic!("No common elements")
        }
    }
    total
}
