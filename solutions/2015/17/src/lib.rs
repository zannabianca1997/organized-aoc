use std::cmp::Reverse;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count(containers: &[usize], quantity: usize) -> usize {
    let mut total = 0;
    for (i, &c) in containers.iter().enumerate() {
        if c == quantity {
            total += 1
        } else if c < quantity {
            total += count(&containers[i + 1..], quantity - c)
        }
    }
    total
}

pub fn part1(input: &str) -> usize {
    count(&parse(input), 150)
}

fn count_min(
    containers: &[usize],
    quantity: usize,
    used: usize,
    best_used: usize,
) -> (usize, usize) {
    let mut total = (best_used, 0);
    for (i, &c) in containers.iter().enumerate() {
        if c == quantity {
            if total.0 > used + 1 {
                total.0 = used + 1;
                total.1 = 1;
            } else if total.0 == used + 1 {
                total.1 += 1
            }
        } else if c < quantity && total.0 >= used + 2 {
            let subtotal = count_min(&containers[i + 1..], quantity - c, used + 1, total.0);
            if total.0 > subtotal.0 {
                total = subtotal
            } else if total.0 == subtotal.0 {
                total.1 += subtotal.1
            }
        }
    }
    total
}

pub fn part2(input: &str) -> usize {
    let mut containers = parse(input);
    containers.sort_unstable_by_key(|n| Reverse(*n));
    count_min(&containers, 150, 0, usize::MAX).1
}
