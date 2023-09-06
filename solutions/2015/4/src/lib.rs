#![feature(generic_const_exprs)]

pub fn part1(input: &str) -> i64 {
    mine::<5>(input)
}

pub fn part2(input: &str) -> i64 {
    mine::<6>(input)
}

fn mine<const N: usize>(input: &str) -> i64
where
    [(); N / 2]:,
{
    for i in 1i64.. {
        let hash: [u8; 16] = md5::compute(format!("{input}{i}").as_bytes()).into();
        if hash[..N / 2] == [0; N / 2]
            && if N % 2 != 0 {
                hash[N / 2] & 0xF0 == 0
            } else {
                true
            }
        {
            return i;
        }
    }
    unreachable!()
}
