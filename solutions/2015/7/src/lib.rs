use std::collections::BTreeMap;

use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BinOp {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}
impl BinOp {
    fn parse(op: &str) -> BinOp {
        match op {
            "AND" => BinOp::AND,
            "OR" => BinOp::OR,
            "LSHIFT" => BinOp::LSHIFT,
            "RSHIFT" => BinOp::RSHIFT,
            _ => panic!("unknow operand: {}", op),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operand<'i> {
    Ident(&'i str),
    Const(u16),
}
impl Operand<'_> {
    fn parse(a: &str) -> Operand<'_> {
        match a.parse() {
            Ok(a) => Operand::Const(a),
            Err(_) => Operand::Ident(a),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Expr<'i> {
    Const(Operand<'i>),
    NOT(Operand<'i>),
    BinOp(BinOp, Operand<'i>, Operand<'i>),
}

#[derive(Debug, Clone)]
struct Input<'i> {
    wires: BTreeMap<&'i str, Expr<'i>>,
}
impl<'i> Input<'i> {
    fn parse(input: &str) -> Input<'_> {
        Input {
            wires: input
                .trim()
                .lines()
                .map(|l| {
                    let (a, b) = l.split_once("->").unwrap();
                    let expr_parts: ArrayVec<_, 3> = a.split_ascii_whitespace().collect();
                    (
                        b.trim(),
                        match *expr_parts {
                            [a] => Expr::Const(Operand::parse(a)),
                            ["NOT", a] => Expr::NOT(Operand::parse(a)),
                            [a, op, b] => {
                                Expr::BinOp(BinOp::parse(op), Operand::parse(a), Operand::parse(b))
                            }
                            _ => panic!("Bad expression {a}"),
                        },
                    )
                })
                .collect(),
        }
    }

    fn eval(&self, wire: &'i str, cache: &mut BTreeMap<&'i str, u16>) -> u16 {
        if let Some(v) = cache.get(wire) {
            return *v;
        }
        let mut eval_op = |op: &Operand<'i>| match op {
            Operand::Ident(a) => self.eval(a, cache),
            Operand::Const(v) => *v,
        };
        let res = match self
            .wires
            .get(wire)
            .expect("All wires should be accounted for")
        {
            Expr::Const(a) => eval_op(a),
            Expr::NOT(a) => !eval_op(a),
            Expr::BinOp(op, a, b) => {
                let a = eval_op(a);
                let b = eval_op(b);
                match op {
                    BinOp::AND => a & b,
                    BinOp::OR => a | b,
                    BinOp::LSHIFT => a << b,
                    BinOp::RSHIFT => a >> b,
                }
            }
        };
        cache.insert(wire, res);
        res
    }
}

pub fn part1(input: &str) -> u16 {
    let input = Input::parse(input);
    input.eval("a", &mut BTreeMap::new())
}

pub fn part2(input: &str) -> u16 {
    let mut input = Input::parse(input);
    let a = input.eval("a", &mut BTreeMap::new());
    *input.wires.get_mut("b").unwrap() = Expr::Const(Operand::Const(a));
    input.eval("a", &mut BTreeMap::new())
}
