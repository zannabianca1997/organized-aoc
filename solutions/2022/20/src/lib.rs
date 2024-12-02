fn parse_input(input: &str) -> Box<[isize]> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> isize {
    let input = parse_input(input);
    let mut list: Box<[_]> = input.iter().map(|v| (*v, true)).collect();

    // decrypt
    for i in 0..list.len() {
        while list[i].1 {
            // this is still unmoved
            let new_i = (i as isize + list[i].0).rem_euclid(list.len() as isize - 1) as usize;

            // scroll the array
            if i < new_i {
                list[i..=new_i].rotate_left(1);
            } else if new_i < i {
                list[new_i..=i].rotate_right(1);
            }
            list[new_i].1 = false;
        }
    }

    // find the 0
    let idx_0 = list
        .iter()
        .position(|v| v.0 == 0)
        .expect("Missing the value 0");
    let p1 = (idx_0 + 1000) % input.len();
    let p2 = (idx_0 + 2000) % input.len();
    let p3 = (idx_0 + 3000) % input.len();

    list[p1].0 + list[p2].0 + list[p3].0
}

pub fn part2(input: &str) -> isize {
    let input = parse_input(input);
    let mut list: Vec<_> = input
        .iter()
        .enumerate()
        .map(|v| (v.0, v.1 * 811589153))
        .collect();

    // decrypt
    for _ in 0..10 {
        for n in 0..list.len() {
            let i = list.iter().position(|(p, _)| *p == n).unwrap();

            // calculate new position
            let new_i = (i as isize + list[i].1).rem_euclid(list.len() as isize - 1) as usize;

            // scroll the array
            if i < new_i {
                list[i..=new_i].rotate_left(1);
            } else if new_i < i {
                list[new_i..=i].rotate_right(1);
            }
        }
    }

    // find the 0
    let idx_0 = list
        .iter()
        .position(|v| v.1 == 0)
        .expect("Missing the value 0");
    let p1 = (idx_0 + 1000) % input.len();
    let p2 = (idx_0 + 2000) % input.len();
    let p3 = (idx_0 + 3000) % input.len();

    list[p1].1 + list[p2].1 + list[p3].1
}
