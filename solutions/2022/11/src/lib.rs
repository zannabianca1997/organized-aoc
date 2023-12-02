#![feature(unwrap_infallible)]
#![feature(never_type)]

use lazy_static::lazy_static;
use regex::Regex;

use std::{
    collections::{hash_map::RandomState, BinaryHeap, HashSet, VecDeque},
    str::FromStr,
};
#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}
impl Operation {
    fn apply(&self, n: usize) -> usize {
        match self {
            Operation::Add(v) => n + v,
            Operation::Mul(v) => n * v,
            Operation::Square => n * n,
        }
    }
}
#[derive(Debug)]
struct MonkeyData {
    items: VecDeque<usize>,
    operation: Operation,
    test_divisor: usize,
    throw_if_true: usize,
    throw_if_false: usize,
    inspections: usize,
}
impl MonkeyData {
    fn turn(&mut self) -> [(usize, VecDeque<usize>); 2] {
        let mut thrown = [
            (self.throw_if_true, VecDeque::new()),
            (self.throw_if_false, VecDeque::new()),
        ];
        for item in self.items.drain(..) {
            // inspection and relaxing
            let item = self.operation.apply(item) / 3;
            self.inspections += 1;
            // throwning
            thrown[if item % self.test_divisor == 0 { 0 } else { 1 }]
                .1
                .push_back(item)
        }
        thrown
    }
    fn turn2(&mut self) -> [(usize, VecDeque<usize>); 2] {
        let mut thrown = [
            (self.throw_if_true, VecDeque::new()),
            (self.throw_if_false, VecDeque::new()),
        ];
        for item in self.items.drain(..) {
            // inspection and relaxing
            let item = self.operation.apply(item);
            self.inspections += 1;
            // throwning
            thrown[if item % self.test_divisor == 0 { 0 } else { 1 }]
                .1
                .push_back(item)
        }
        thrown
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<MonkeyData>,
    modulus: usize,
}
impl Monkeys {
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let thrown = self.monkeys[i].turn();
            for (dest, items) in thrown {
                self.monkeys[dest].items.extend(items)
            }
        }
    }
    fn round2(&mut self) {
        for i in 0..self.monkeys.len() {
            let thrown = self.monkeys[i].turn2();
            for (dest, items) in thrown {
                self.monkeys[dest]
                    .items
                    .extend(items.into_iter().map(|v| v % self.modulus))
            }
        }
    }
    fn monkey_business(&self) -> usize {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<BinaryHeap<_>>();
        let max = inspections
            .pop()
            .expect("There should be at least one monkey");
        let second = inspections
            .pop()
            .expect("There should be at least two monkey");
        max * second
    }
}
impl FromStr for Monkeys {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MONKEY_RE: Regex = Regex::new(r"Monkey\s+(?P<num>\d+)\s*:\s+Starting items\s*:\s*(?P<items>\d+(?:\s*,\s*\d+)*)\s+Operation\s*:\s*new\s*=\s*old\s*(?P<op>[*+])\s*(?P<op_value>\d+|old)\s+Test\s*:\s*divisible by\s+(?P<test_divisor>\d+)\s+If true\s*:\sthrow to monkey\s+(?P<throw_if_true>\d+)\s+If false\s*:\sthrow to monkey\s+(?P<throw_if_false>\d+)").unwrap();
        }
        // collecting monkey regex matches
        let mut monkeys: Vec<(usize, _)> = MONKEY_RE
            .captures_iter(s)
            .map(|capture| {
                (
                    capture.name("num").unwrap().as_str().parse().unwrap(),
                    MonkeyData {
                        items: capture
                            .name("items")
                            .unwrap()
                            .as_str()
                            .split(",")
                            .map(|v| v.trim().parse().unwrap())
                            .collect(),
                        operation: match (
                            capture.name("op").unwrap().as_str(),
                            capture.name("op_value").unwrap().as_str(),
                        ) {
                            ("+", v) => Operation::Add(v.parse().unwrap()),
                            ("*", "old") => Operation::Square,
                            ("*", v) => Operation::Mul(v.parse().unwrap()),
                            _ => unreachable!(),
                        },
                        test_divisor: capture
                            .name("test_divisor")
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                        throw_if_true: capture
                            .name("throw_if_true")
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                        throw_if_false: capture
                            .name("throw_if_false")
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                        inspections: 0,
                    },
                )
            })
            .collect();
        // sorting monkeys
        monkeys.sort_by_key(|(v, _)| *v);
        // checking all monkey are accounted for
        if monkeys
            .iter()
            .enumerate()
            .all(|(idx, (mnum, _))| idx == *mnum)
        {
            // discarding indices
            let monkeys: Vec<_> = monkeys.into_iter().map(|(_, m)| m).collect();
            // calculating modulus
            let modulus = monkeys.iter().map(|m| m.test_divisor).product();
            Ok(Self { monkeys, modulus })
        } else {
            let monkeys: HashSet<_, RandomState> =
                HashSet::from_iter(monkeys.into_iter().map(|(v, _)| v));
            let missing: Vec<_> = HashSet::from_iter(0..=*monkeys.iter().max().unwrap())
                .difference(&monkeys)
                .map(|v| *v)
                .collect();
            panic!("Missing monkeys {missing:?}")
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut monkeys: Monkeys = input.parse().into_ok();
    for _ in 0..20 {
        monkeys.round()
    }
    monkeys.monkey_business()
}

pub fn part2(input: &str) -> usize {
    let mut monkeys: Monkeys = input.parse().into_ok();
    for _ in 0..10000 {
        monkeys.round2()
    }
    monkeys.monkey_business()
}
