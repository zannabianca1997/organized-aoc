#![feature(unwrap_infallible)]
#![feature(never_type)]
use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;

include! {env!("ELEMENTS_RS")}

fn look_and_say(seq: &mut Vec<u8>) {
    let Some(first) = seq.first().copied() else {
        // nothing to do on an empty sequence
        return;
    };
    let mut new_seq = vec![(first, 0usize)];
    for i in seq.drain(..) {
        if new_seq.last().unwrap().0 == i {
            new_seq.last_mut().unwrap().1 += 1;
        } else {
            new_seq.push((i, 1))
        }
    }
    *seq = new_seq
        .into_iter()
        .format_with("", |(ch, n), f| {
            f(&n)?;
            f(&(ch as char))
        })
        .to_string()
        .into();
}

struct Sequence {
    prefix: Vec<u8>,
    elements: BTreeMap<Element, usize>,
}
impl Sequence {
    fn len(&self) -> usize {
        self.prefix.len()
            + self
                .elements
                .iter()
                .map(|(e, n)| e.sequence().len() * *n)
                .sum::<usize>()
    }

    fn step(self) -> Self {
        let Self {
            mut prefix,
            mut elements,
        } = self;

        'cutting: while !prefix.is_empty() {
            for i in 0..prefix.len() - 1 {
                // is there an element with this sequence?
                if let Some(element) = Element::from_seq(&prefix[i..]) {
                    // is it permanently separated from the rest?
                    if i == 0 || !element.start_set().contains(&prefix[i - 1]) {
                        prefix.truncate(i);
                        *elements.entry(element).or_insert(0) += 1;
                        // i need to restart the for cycle
                        continue 'cutting;
                    }
                }
            }
            // no possible cut found, advancing remaining prefix
            look_and_say(&mut prefix);
            break 'cutting;
        }

        let mut new_elements = BTreeMap::new();
        for (element, count) in elements {
            for &d in element.decay() {
                *new_elements.entry(d).or_insert(0) += count
            }
        }

        Self {
            prefix,
            elements: new_elements,
        }
    }
}
impl FromStr for Sequence {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            prefix: s.into(),
            elements: BTreeMap::new(),
        })
    }
}

pub fn part1(input: &str) -> usize {
    let mut seq: Sequence = input.parse().into_ok();
    for _ in 0..40 {
        seq = seq.step()
    }
    seq.len()
}

pub fn part2(input: &str) -> usize {
    let mut seq: Sequence = input.parse().into_ok();
    for _ in 0..50 {
        seq = seq.step()
    }
    seq.len()
}

#[cfg(test)]
mod test {
    use crate::look_and_say;

    #[test]
    fn raw_look_and_say() {
        let mut seq = b"1".to_vec();
        look_and_say(&mut seq);
        assert_eq!(seq.as_slice(), b"11");
        look_and_say(&mut seq);
        assert_eq!(seq.as_slice(), b"21");
        look_and_say(&mut seq);
        assert_eq!(seq.as_slice(), b"1211");
        look_and_say(&mut seq);
        assert_eq!(seq.as_slice(), b"111221");
        look_and_say(&mut seq);
        assert_eq!(seq.as_slice(), b"312211");
    }
}
