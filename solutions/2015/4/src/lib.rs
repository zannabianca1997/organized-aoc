pub fn part1(input: &str) -> usize {
    mine::<2, true>(input)
}

pub fn part2(input: &str) -> usize {
    mine::<3, false>(input)
}

fn mine<const N_HALF: usize, const N_ODD: bool>(input: &str) -> usize {
    for i in 1.. {
        let hash: [u8; 16] = md5::compute(format!("{input}{i}").as_bytes()).into();
        if hash[..N_HALF] == [0; N_HALF]
            && if N_ODD {
                hash[N_HALF] & 0xF0 == 0
            } else {
                true
            }
        {
            return i;
        }
    }
    unreachable!()
}
