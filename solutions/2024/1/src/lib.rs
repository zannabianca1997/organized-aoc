#![feature(iter_next_chunk)]

pub fn part1(input: &str) -> usize {
    let (mut rlist, mut llist) = lists(input);
    rlist.sort_unstable();
    llist.sort_unstable();
    Iterator::zip(llist.into_iter(), rlist)
        .map(|(r, l)| r.abs_diff(l))
        .sum()
}

fn lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut rlist: Vec<usize> = Vec::new();
    let mut llist: Vec<usize> = Vec::new();
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let [r, l] = line.split_whitespace().next_chunk().unwrap();
        rlist.push(r.parse().unwrap());
        llist.push(l.parse().unwrap());
    }
    (rlist, llist)
}

pub fn part2(input: &str) -> usize {
    let (rlist, llist) = lists(input);
    llist
        .iter()
        .map(|l| rlist.iter().filter(|r| *r == l).count() * l)
        .sum()
}
