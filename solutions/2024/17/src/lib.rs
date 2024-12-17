#![feature(slice_as_chunks)]
#![feature(slice_split_once)]

use lazy_regex::regex_captures;
use std::{fmt::Write as _, iter::once};

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone)]
struct State {
    regs: Registers,
    program: Box<[u8]>,
}

fn combo(a: u8, regs: &Registers) -> u64 {
    match a {
        0..=3 => a as _,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!(),
    }
}

fn parse(input: &str) -> State {
    let (_, a, b, c, program) = regex_captures!(
        r"^Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (\d+(?:,\d+)*)\n$",
        input
    )
    .unwrap();

    State {
        regs: Registers {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
        },
        program: program.split(',').map(|v| v.parse().unwrap()).collect(),
    }
}

pub fn part1(input: &str) -> String {
    let State { mut regs, program } = parse(input);

    let mut ip = 0;
    let mut out = String::new();
    let mut output = |v: u64| write!(&mut out, "{v},").unwrap();

    while ip < program.len() {
        let [opcode, operand] = *program[ip..].first_chunk().unwrap();

        #[cfg(debug_assertions)]
        eprintln!(
            "{ip:4}: {} {operand}\t(a: {:o}\tb: {:o}\tc: {:o}) ",
            match opcode {
                0 => "adv",
                1 => "bxl",
                2 => "bst",
                3 => "jnz",
                4 => "bxc",
                5 => "out",
                6 => "bdv",
                7 => "cdv",
                _ => panic!(),
            },
            regs.a,
            regs.b,
            regs.c,
        );

        let mut jumped = false;

        match opcode {
            0 => regs.a >>= combo(operand, &regs),
            1 => regs.b ^= operand as u64,
            2 => regs.b = combo(operand, &regs) & 0b111,
            3 => {
                if regs.a != 0 {
                    ip = operand as _;
                    jumped = true
                }
            }
            4 => regs.b ^= regs.c,
            5 => output(combo(operand, &regs) & 0b111),
            6 => regs.b = regs.a >> combo(operand, &regs),
            7 => regs.c = regs.a >> combo(operand, &regs),
            _ => panic!(),
        }

        if !jumped {
            ip += 2
        }
    }

    if out.ends_with(',') {
        out.pop();
    }

    out
}

/*
Code is :

    bst 4
    bxl 5
    cdv 5
    bxl 6
    adv 3
    bxc 1
    out 5
    jnz 0

Into c:

    do {
        b = a  & 0b111;
        b ^= 5;
        c = a >> b;
        b ^= 6;
        a >>= 3
        b ^= c
        printf("%s,", b  & 0b111);
    } while (a != 0)

Moving around and knowing a is diffent from 0 at start

    for (;a>>=3;a!=0) {
        b = a & 0b111;
        c = a >> (b ^ 0b101);
        b ^= 0b011 ^ c;
        printf("%s,", b  & 0b111);
    }

This is using `a` as an array of octals and taking out element from it.

*/

pub fn part2(input: &str) -> u64 {
    let State { regs, program } = parse(input);

    #[cfg(debug_assertions)]
    {
        assert_eq!(regs.b, 0);
        assert_eq!(regs.c, 0);

        // Check that the program is the kind we expect
        let (instrs, &[]) = program.as_chunks() else {
            panic!("The program has a leftover number at the end")
        };
        let loop_body = &instrs[..instrs.len() - 1];

        // The program is in a loop
        assert_eq!(
            *instrs.last().unwrap(),
            [3, 0],
            "The program does not end with a jump to the start"
        );
        assert!(
            loop_body.iter().all(|[opcode, _]| *opcode != 3),
            "The program has jumps inside it"
        );

        assert_eq!(
            loop_body
                .iter()
                .filter_map(|instr| (instr[0] == 0).then_some(instr[1]))
                .inspect(|shift| assert!(*shift <= 3))
                .sum::<u8>(),
            3,
            "The loop does not shift `a` of three bits"
        );
        assert_eq!(
            loop_body.iter().filter(|instr| instr[0] == 5).count(),
            1,
            "The loop contains multiple outputs"
        );

        let mut is_b_dirty = true;
        let mut is_c_dirty = true;
        for &[opcode, operand] in loop_body {
            match opcode {
                0 => (),
                1 => (),
                2 => {
                    is_b_dirty = match operand {
                        0..=3 => false,
                        4 => false,
                        5 => is_b_dirty,
                        6 => is_c_dirty,
                        _ => panic!(),
                    }
                }
                3 => panic!("No jumps in the loop"),
                4 => is_b_dirty |= is_c_dirty,
                5 => {
                    assert!(
                        match operand {
                            0..=3 => true,
                            4 => true,
                            5 => !is_b_dirty,
                            6 => !is_c_dirty,
                            _ => panic!(),
                        },
                        "The loop output depends on the `b` and `c` values at the start of the loop"
                    );
                }
                6 => {
                    is_b_dirty = match operand {
                        0..=3 => false,
                        4 => false,
                        5 => is_b_dirty,
                        6 => is_c_dirty,
                        _ => panic!(),
                    }
                }
                7 => {
                    is_c_dirty = match operand {
                        0..=3 => false,
                        4 => false,
                        5 => is_b_dirty,
                        6 => is_c_dirty,
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }

        eprintln!("The program conform to the accepted ones")
    }
    #[cfg(not(debug_assertions))]
    let _ = regs;

    // This is the loop body before the `out` instruction
    let mut print_operand = 0;
    let loop_body = program
        .split_last_chunk::<2>()
        .unwrap()
        .0
        .as_chunks::<2>()
        .0
        .split_once(|i| {
            if i[0] == 5 {
                print_operand = i[1];
                true
            } else {
                false
            }
        })
        .unwrap()
        .0;
    let print_operand = print_operand;

    let a = a_value_to_emit(&*program, loop_body, print_operand)
        .next()
        .unwrap();

    a
}

fn a_value_to_emit<'a>(
    to_emit: &'a [u8],
    loop_body: &'a [[u8; 2]],
    print_operand: u8,
) -> Box<dyn Iterator<Item = u64> + 'a> {
    let (&to_emit, left_to_emit) = to_emit.split_first().unwrap();

    Box::new(
        if left_to_emit.is_empty() {
            Box::new(once(0))
        } else {
            a_value_to_emit(left_to_emit, loop_body, print_operand)
        }
        .flat_map(move |a_value_to_emit_rest| {
            (0..7).filter_map(move |a_chunk| {
                let a = a_value_to_emit_rest << 3 | a_chunk;

                let mut regs = Registers { a, b: 0, c: 0 };

                for &[opcode, operand] in loop_body {
                    match opcode {
                        0 => (), // `a` shifting is implemented outside
                        1 => regs.b ^= operand as u64,
                        2 => regs.b = combo(operand, &regs) & 0b111,
                        3 => panic!("No jumps in the loop"),
                        4 => regs.b ^= regs.c,
                        5 => panic!("Output was cut off"),
                        6 => regs.b = regs.a >> combo(operand, &regs),
                        7 => regs.c = regs.a >> combo(operand, &regs),
                        _ => panic!(),
                    }
                }

                let printed = (combo(print_operand, &regs) & 0b111) as u8;

                if printed == to_emit {
                    Some(a)
                } else {
                    None
                }
            })
        }),
    )
}
