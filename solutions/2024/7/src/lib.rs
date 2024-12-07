fn parse(input: &str) -> impl Iterator<Item = (usize, impl Iterator<Item = usize> + Clone + '_)> {
    input.trim().lines().map(|l| {
        let (n, l) = l.split_once(':').unwrap();
        (
            n.trim().parse().unwrap(),
            l.split_whitespace().map(|n| n.parse().unwrap()),
        )
    })
}

pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for (target, mut operands) in parse(input) {
        if can_be_built(target, operands.next().unwrap(), operands) {
            total += target
        }
    }
    total
}

fn can_be_built(
    target: usize,
    current: usize,
    mut operands: impl Iterator<Item = usize> + Clone,
) -> bool {
    // We only have `+` and `*` so the current value can only grow
    // This would fail if a `0` was in the values, but the input does not contain one
    if current > target {
        return false;
    }
    let Some(next) = operands.next() else {
        return target == current;
    };
    can_be_built(target, current + next, operands.clone())
        || can_be_built(target, current * next, operands)
}

fn can_be_built_with_concat(
    target: usize,
    current: usize,
    mut operands: impl Iterator<Item = usize> + Clone,
) -> bool {
    // We have `+`, `*` and `||` so the current value can only grow
    // This would fail if a `0` was in the values, but the input does not contain one
    if current > target {
        return false;
    }
    let Some(next) = operands.next() else {
        return target == current;
    };
    can_be_built_with_concat(target, current + next, operands.clone())
        || can_be_built_with_concat(target, current * next, operands.clone())
        || can_be_built_with_concat(target, concat(current, next), operands)
}

fn concat(a: usize, b: usize) -> usize {
    let b_digits = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10usize.pow(b_digits) + b
}

pub fn part2(input: &str) -> usize {
    let mut total = 0;
    for (target, mut operands) in parse(input) {
        if can_be_built_with_concat(target, operands.next().unwrap(), operands) {
            total += target
        }
    }
    total
}

#[cfg(test)]
mod tests {
    static EXAMPLES: &[(&str, bool, bool)] = &[
        ("190: 10 19", true, true),
        ("3267: 81 40 27", true, true),
        ("83: 17 5", false, false),
        ("156: 15 6", false, true),
        ("7290: 6 8 6 15", false, true),
        ("161011: 16 10 13", false, false),
        ("192: 17 8 14", false, true),
        ("21037: 9 7 18 13", false, false),
        ("292: 11 6 16 20", true, true),
    ];

    mod concat {
        use super::super::concat;

        #[test]
        fn _156() {
            assert_eq!(concat(15, 6), 156)
        }
    }
}
