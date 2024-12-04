use lazy_regex::{regex, Lazy, Regex};

static MUL_RE: &Lazy<Regex> = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
static COND: &Lazy<Regex> = regex!(r"do(n't|)\(\)");

pub fn part1(input: &str) -> usize {
    MUL_RE
        .captures_iter(input)
        .map(|c| {
            let [a, b]: [usize; 2] = c.extract().1.map(|e| e.parse().unwrap());
            a * b
        })
        .sum()
}

pub fn part2(mut input: &str) -> usize {
    let mut active = true;
    let mut acc = 0;
    while !input.is_empty() {
        let next_match = COND.captures(input);
        let chunk;
        let chunk_is_active = active;
        if let Some(next_match) = next_match {
            chunk = &input[..next_match.get(0).unwrap().start()];
            input = &input[next_match.get(0).unwrap().end()..];
            active = next_match.extract::<1>().1[0].is_empty()
        } else {
            chunk = input;
            input = ""
        };
        if chunk_is_active {
            acc += part1(chunk)
        }
    }
    acc
}
