pub fn antennas(input: &str) -> impl Iterator<Item = (usize, usize, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(x, ch)| (ch != '.').then_some((x, y, ch)))
    })
}

pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let antennas = antennas(input).fold(fnv::FnvHashMap::default(), |mut map, (x, y, t)| {
        map.entry(t).or_insert_with(|| vec![]).push((x, y));
        map
    });
    let mut antinodes = fnv::FnvHashSet::default();
    for antennas in antennas.values() {
        for i in 0..antennas.len() {
            for j in 0..i {
                let a1 = antennas[i];
                let a2 = antennas[j];

                let n1 = ((2 * a1.0).wrapping_sub(a2.0), (2 * a1.1).wrapping_sub(a2.1));
                let n2 = ((2 * a2.0).wrapping_sub(a1.0), (2 * a2.1).wrapping_sub(a1.1));

                antinodes.extend(
                    [n1, n2]
                        .into_iter()
                        .filter(|(x, y)| *x < width && *y < height),
                );
            }
        }
    }
    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let antennas = antennas(input).fold(fnv::FnvHashMap::default(), |mut map, (x, y, t)| {
        map.entry(t).or_insert_with(|| vec![]).push((x, y));
        map
    });
    let mut antinodes = fnv::FnvHashSet::default();
    for antennas in antennas.values() {
        for i in 0..antennas.len() {
            for j in 0..i {
                let a1 = antennas[i];
                let a2 = antennas[j];

                let n1 = (0..)
                    .map(|i| {
                        (
                            ((1 + i) * a1.0).wrapping_sub(i * a2.0),
                            ((1 + i) * a1.1).wrapping_sub(i * a2.1),
                        )
                    })
                    .take_while(|(x, y)| *x < width && *y < height);
                let n2 = (0..)
                    .map(|i| {
                        (
                            ((1 + i) * a2.0).wrapping_sub(i * a1.0),
                            ((1 + i) * a2.1).wrapping_sub(i * a1.1),
                        )
                    })
                    .take_while(|(x, y)| *x < width && *y < height);

                antinodes.extend(n1.chain(n2));
            }
        }
    }
    antinodes.len()
}
