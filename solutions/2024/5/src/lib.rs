use std::cmp::Ordering;

use fnv::FnvHashSet;

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
    let rules: FnvHashSet<_> = rules.collect();
    let order_by_rules = comparing_fn(&rules);
    updates
        .filter_map(|update| {
            let update: Box<[_]> = update.collect();
            update
                .is_sorted_by(order_by_rules)
                .then(|| update[update.len() / 2])
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let rules: FnvHashSet<_> = rules.collect();
    let order_by_rules = comparing_fn(&rules);
    let sort_by_rules = sorting_fn(&rules);
    updates
        .filter_map(|update| {
            let update: Box<[_]> = update.collect();
            if update.is_sorted_by(order_by_rules) {
                return None;
            }
            // This is an unordered line
            let mut update = update;
            update.sort_unstable_by(sort_by_rules);

            Some(update[update.len() / 2])
        })
        .sum()
}

fn comparing_fn(rules: &FnvHashSet<(usize, usize)>) -> impl Fn(&usize, &usize) -> bool + Copy + '_ {
    |a, b| !rules.contains(&(*b, *a))
}
fn sorting_fn(
    rules: &FnvHashSet<(usize, usize)>,
) -> impl Fn(&usize, &usize) -> Ordering + Copy + '_ {
    |a, b| {
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
    }
}
