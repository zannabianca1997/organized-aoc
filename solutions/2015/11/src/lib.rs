#![feature(array_windows)]

fn parse(input: &str) -> [u8; 8] {
    input
        .as_bytes()
        .try_into()
        .expect("Input shoudld be of 8 characters")
}

/// increment a password, skipping `i`, `o` and `l`
fn step(mut psw: [u8; 8]) -> [u8; 8] {
    let mut idx = 7;
    while psw[idx] == b'z' {
        psw[idx] = b'a';
        if idx == 0 {
            return psw;
        }
        idx -= 1;
    }
    psw[idx] = match psw[idx] {
        b'h' | b'i' => b'j',
        b'k' | b'l' => b'm',
        b'n' | b'o' => b'p',
        _ => psw[idx] + 1,
    };
    psw
}

/// Fix a password that may contains `i`, `o` and `l`
/// returning the first one after without them
fn fix_or_step(mut psw: [u8; 8]) -> [u8; 8] {
    for i in 0..8 {
        if [b'i', b'o', b'l'].contains(&psw[i]) {
            psw[i] += 1;
            for ch in &mut psw[i + 1..] {
                *ch = b'a'
            }
            return psw;
        }
    }
    step(psw)
}

fn check(psw: &[u8; 8]) -> bool {
    if !psw
        .array_windows()
        .any(|[a, b, c]| a + 1 == *b && b + 1 == *c)
    {
        return false;
    }
    for i in 0..5 {
        if psw[i] == psw[i + 1] {
            for j in i + 2..7 {
                if psw[j] == psw[j + 1] && psw[i] != psw[j] {
                    return true;
                }
            }
        }
    }
    false
}

/// Find the first valid password at or after `psw`
fn first_valid(mut psw: [u8; 8]) -> [u8; 8] {
    while !check(&psw) {
        psw = step(psw);
    }
    psw
}

pub fn part1(input: &str) -> [u8; 8] {
    let psw = parse(input);
    first_valid(fix_or_step(psw))
}

pub fn part2(input: &str) -> [u8; 8] {
    first_valid(step(part1(input)))
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn abcdefgh() {
        assert_eq!(
            String::from_utf8_lossy(&part1("abcdefgh")),
            String::from_utf8_lossy(&parse("abcdffaa"))
        )
    }
    #[test]
    fn ghijklmn() {
        assert_eq!(
            String::from_utf8_lossy(&part1("ghijklmn")),
            String::from_utf8_lossy(&parse("ghjaabcc"))
        )
    }

    mod check {
        use crate::{check, parse};

        #[test]
        fn abcdffaa() {
            assert!(check(&parse("abcdffaa")))
        }
        #[test]
        fn ghjaabcc() {
            assert!(check(&parse("ghjaabcc")))
        }
    }
}
