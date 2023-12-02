#![feature(never_type)]
#![feature(unwrap_infallible)]
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Counts {
    red: usize,
    green: usize,
    blue: usize,
}
impl Counts {
    fn possible_with_total(self, total: Counts) -> bool {
        self.red <= total.red && self.blue <= total.blue && self.green <= total.green
    }
    fn power(self) -> usize {
        self.red * self.green * self.blue
    }
}
impl FromStr for Counts {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        for s in s.split(',') {
            let (num, color) = s.trim().split_once(' ').unwrap();
            let num = num.trim().parse().unwrap();
            match color.trim() {
                "red" => res.red = num,
                "blue" => res.blue = num,
                "green" => res.green = num,
                _ => panic!(),
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    observations: Vec<Counts>,
}
impl Game {
    fn possible_with_total(&self, total: Counts) -> bool {
        self.observations
            .iter()
            .all(|o| o.possible_with_total(total))
    }
    fn minimum_total(&self) -> Counts {
        Counts {
            red: self.observations.iter().map(|o| o.red).max().unwrap(),
            green: self.observations.iter().map(|o| o.green).max().unwrap(),
            blue: self.observations.iter().map(|o| o.blue).max().unwrap(),
        }
    }
}
impl FromStr for Game {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game").unwrap();
        let (id, observations) = s.split_once(':').unwrap();
        let id = id.trim().parse().unwrap();
        let observations = observations
            .split(';')
            .map(|o| o.trim().parse().into_ok())
            .collect();
        Ok(Self { id, observations })
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Game>().into_ok())
        .filter(|g| {
            g.possible_with_total(Counts {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Game>().into_ok().minimum_total().power())
        .sum()
}
