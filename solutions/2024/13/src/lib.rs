use lazy_regex::{regex, Lazy, Regex};
use nalgebra::{Matrix2, Vector2};

struct Claw {
    ab: Matrix2<i64>,
    p: Vector2<i64>,
}

fn parse(input: &str) -> impl Iterator<Item = Claw> + '_ {
    static RE: &Lazy<Regex> = regex!(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)"
    );
    RE.captures_iter(input).map(|capture| {
        let [ax, ay, bx, by, px, py] = capture.extract().1.map(|v| v.parse().unwrap());
        Claw {
            ab: Matrix2::new(ax, bx, ay, by),
            p: Vector2::new(px, py),
        }
    })
}

fn tokens(Claw { ab, p }: Claw) -> Option<u64> {
    let det = ab[(0, 0)] * ab[(1, 1)] - ab[(0, 1)] * ab[(1, 0)];
    if det == 0 {
        // Not invertible
        return None;
    }
    let inverse = Matrix2::new(ab[(1, 1)], -ab[(0, 1)], -ab[(1, 0)], ab[(0, 0)]);

    let [Some(a), Some(b)] = <[_; 2]>::from(inverse * p).map(|v| {
        if v % det != 0 {
            // Not divisible
            return None;
        }
        if v.signum() * det.signum() < 0 {
            // division would give negative pushes
            return None;
        }
        Some(v)
    }) else {
        return None;
    };

    Some(((3 * a + b) / det) as u64)
}

pub fn part1(input: &str) -> u64 {
    parse(input).filter_map(tokens).sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|mut claw| {
            claw.p += Vector2::new(10000000000000, 10000000000000);
            claw
        })
        .filter_map(tokens)
        .sum()
}

#[test]
fn test_1() {
    debug_assert_eq!(
        part1(
            r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
        ),
        280
    )
}
