fn elves_backpacks(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n\n")
        .map(|pack| {
            pack.split("\n")
                .filter(|line| line.trim() != "")
                .map(|line| line.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    elves_backpacks(input)
        .into_iter()
        .map(|pack| pack.into_iter().sum())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut packs = elves_backpacks(input)
        .into_iter()
        .map(|pack| pack.into_iter().sum::<i64>());
    let mut largest_three = [
        packs.next().unwrap(),
        packs.next().unwrap(),
        packs.next().unwrap(),
    ];
    largest_three.sort();
    for pack in packs {
        if pack > largest_three[0] {
            largest_three[0] = pack;
            largest_three.sort();
        }
    }
    largest_three.into_iter().sum()
}
