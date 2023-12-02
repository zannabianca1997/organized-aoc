#![feature(never_type)]
use std::{cmp::Ordering, iter::Peekable};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Num(usize),
    List(Vec<Item>),
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Item::*;
        match (self, other) {
            (Num(n1), Num(n2)) => {
                return n1.cmp(n2);
            }
            (Num(_), List(_)) => {
                return Item::List(vec![(*self).clone()]).cmp(other);
            }
            (List(_), Num(_)) => {
                return self.cmp(&Item::List(vec![(*other).clone()]));
            }
            (List(v1), List(v2)) => {
                for idx in 0.. {
                    match (v1.get(idx), v2.get(idx)) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => (),
                            ord => {
                                return ord;
                            }
                        },
                    }
                }
            }
        }
        unreachable!()
    }
}
impl Item {
    fn from_chars<ChIter>(chs: &mut Peekable<ChIter>) -> Self
    where
        ChIter: Iterator<Item = char>,
    {
        match chs.peek() {
            Some('[') => {
                chs.next();
                if let Some(']') = chs.peek() {
                    chs.next();
                    Self::List(vec![])
                } else {
                    let mut items = vec![Self::from_chars(chs)];
                    while let Some(',') = chs.peek() {
                        chs.next();
                        items.push(Self::from_chars(chs));
                    }
                    if let Some(']') = chs.next() {
                        Self::List(items)
                    } else {
                        panic!("Expected ]")
                    }
                }
            }
            Some(ch) if ch.is_digit(10) => {
                let mut digits = String::new();
                while chs.peek().is_some_and(|ch| ch.is_digit(10)) {
                    digits.push(chs.next().unwrap())
                }
                let n = digits.parse::<usize>().unwrap();
                Self::Num(n)
            }
            Some(ch) => panic!("Unexpected char {ch:?}"),
            None => panic!("Expected Items"),
        }
    }
}
impl From<&str> for Item {
    fn from(value: &str) -> Self {
        let chs = &mut value.chars().peekable();
        let res = Self::from_chars(chs);
        if let None = chs.next() {
            res
        } else {
            panic!("String not consumed")
        }
    }
}

fn parse_input(input: &str) -> Vec<(Item, Item)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let (p1, p2) = pair.split_once("\n").unwrap();
            (Item::from(p1), Item::from(p2))
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Item> {
    input
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.into())
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| (p1 < p2).then_some(i + 1))
        .sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let mut input = parse_input_2(input);
    let marker_2 = Item::try_from("[[2]]").unwrap();
    let marker_6 = Item::try_from("[[6]]").unwrap();
    input.push(marker_2.clone());
    input.push(marker_6.clone());
    input.sort();
    (input.binary_search(&marker_2).unwrap() + 1) * (input.binary_search(&marker_6).unwrap() + 1)
}
