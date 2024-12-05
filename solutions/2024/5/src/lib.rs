use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, DefaultHasher, RandomState},
};

pub fn parse(
    input: &str,
) -> (
    impl Iterator<Item = (usize, usize)> + '_,
    impl Iterator<Item = impl Iterator<Item = usize> + '_>,
) {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();
    let rules = rules.lines().map(|l| {
        let (a, b) = l.split_once('|').unwrap();
        let [a, b] = [a, b].map(|p| p.parse().unwrap());
        (a, b)
    });
    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()));
    (rules, updates)
}

pub fn part1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let rules: Box<[_]> = rules.collect();
    updates
        .filter_map(|update| {
            let update: Box<[_]> = update.collect();
            let pos: HashMap<_, _, BuildHasherDefault<DefaultHasher>> =
                HashMap::from_iter(update.iter().copied().enumerate().map(|(i, v)| (v, i)));
            for (a, b) in &rules {
                if let (Some(pa), Some(pb)) = (pos.get(a), pos.get(b)) {
                    if pa > pb {
                        return None;
                    }
                }
            }
            Some(update[update.len() / 2])
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let rules: HashSet<_, BuildHasherDefault<DefaultHasher>> = rules.collect();
    updates
        .filter_map(|update| {
            let update: Box<[_]> = update.collect();
            let pos: HashMap<_, _, BuildHasherDefault<DefaultHasher>> =
                HashMap::from_iter(update.iter().copied().enumerate().map(|(i, v)| (v, i)));
            let mut ordered = true;
            for (a, b) in &rules {
                if let (Some(pa), Some(pb)) = (pos.get(a), pos.get(b)) {
                    if pa > pb {
                        ordered = false;
                    }
                }
            }
            if ordered {
                return None;
            }
            // This is an unordered line
            let mut update = update;
            update.sort_unstable_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    // We know that a should come before b, so it must be less
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    // We know that a should come after b, so it must be greather
                    Ordering::Greater
                } else {
                    // We don't care how you order those two
                    Ordering::Equal
                }
            });

            Some(update[update.len() / 2])
        })
        .sum()
}
