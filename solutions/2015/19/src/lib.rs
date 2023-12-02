use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<(usize, Vec<usize>)>,
    medicine: Vec<usize>,
    _electron: usize,
}

struct IterElements<'i>(&'i [u8]);

impl<'i> Iterator for IterElements<'i> {
    type Item = &'i [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let len = if self.0[0] == b'e' {
            1
        } else if self.0[0].is_ascii_uppercase() {
            if self.0.len() >= 2 && self.0[1].is_ascii_lowercase() {
                2
            } else {
                1
            }
        } else {
            panic!("All elements should be single uppercase letters, or uppercase then lowercase, or electrons")
        };
        let res = &self.0[..len];
        self.0 = &self.0[len..];
        Some(res)
    }
}

fn parse(input: &str) -> Input {
    let mut elements = HashMap::new();
    let mut ecount: usize = 0;
    let mut input = input.lines();
    let mut rules = vec![];
    while let Some(rule) = input.next() {
        if rule.trim().is_empty() {
            break;
        }
        let (element, decay) = rule.split_once("=>").unwrap();
        let element = *elements
            .entry(element.trim().as_bytes())
            .or_insert_with(|| {
                let val = ecount;
                ecount += 1;
                val
            });
        let decay: Vec<_> = IterElements(decay.trim().as_bytes())
            .map(|el| {
                *elements.entry(el).or_insert_with(|| {
                    let val = ecount;
                    ecount += 1;
                    val
                })
            })
            .collect();
        rules.push((element, decay))
    }
    let medicine: Vec<_> = IterElements(input.next().unwrap().as_bytes())
        .map(|el| {
            *elements.entry(el).or_insert_with(|| {
                let val = ecount;
                ecount += 1;
                val
            })
        })
        .collect();
    let electron = *elements.entry(b"e").or_insert_with(|| {
        let val = ecount;
        ecount += 1;
        val
    });
    Input {
        rules,
        medicine,
        _electron: electron,
    }
}

pub fn part1(input: &str) -> usize {
    let Input {
        rules, medicine, ..
    } = parse(input);
    let mut subbed = HashSet::new();
    for (idx, element) in medicine.iter().enumerate() {
        for (_, decay) in rules.iter().filter(|(decaing, _)| decaing == element) {
            let mut decaied = Vec::with_capacity(medicine.len() - 1 + decay.len());
            decaied.extend_from_slice(&medicine[..idx]);
            decaied.extend_from_slice(&decay);
            decaied.extend_from_slice(&medicine[idx + 1..]);
            subbed.insert(decaied);
        }
    }
    subbed.len()
}
