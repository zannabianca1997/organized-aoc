pub fn part1(input: &str) -> i64 {
    mine::<2, true>(input)
}

pub fn part2(input: &str) -> i64 {
    mine::<3, false>(input)
}

fn mine<const N_HALF: usize, const N_ODD: bool>(input: &str) -> i64 {
    for i in 1i64.. {
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
