fn parse(input: &str) -> impl Iterator<Item = [usize; 3]> + '_ {
    input.trim().lines().map(|l| {
        let mut l = l.splitn(3, 'x');
        [
            l.next().unwrap().parse().unwrap(),
            l.next().unwrap().parse().unwrap(),
            l.next().unwrap().parse().unwrap(),
        ]
    })
}

fn min3<T>(a: T, b: T, c: T) -> T
where
    T: Ord,
{
    T::min(a, T::min(b, c))
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|[l, w, h]| 2 * l * w + 2 * w * h + 2 * h * l + min3(l * w, w * h, h * l))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|[l, w, h]| 2 * min3(l + w, w + h, h + l) + l * w * h)
        .sum()
}
