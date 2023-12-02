use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

#[derive(Debug)]
enum MonkeIntruction<'inp> {
    Const(isize),
    Sum(&'inp str, &'inp str),
    Sub(&'inp str, &'inp str),
    Mul(&'inp str, &'inp str),
    Div(&'inp str, &'inp str),
}
impl MonkeIntruction<'_> {
    fn yell(&self, monkeys: &HashMap<&str, MonkeIntruction>) -> isize {
        match self {
            MonkeIntruction::Const(a) => *a,
            MonkeIntruction::Sum(a, b) => monkeys[a].yell(monkeys) + monkeys[b].yell(monkeys),
            MonkeIntruction::Sub(a, b) => monkeys[a].yell(monkeys) - monkeys[b].yell(monkeys),
            MonkeIntruction::Mul(a, b) => monkeys[a].yell(monkeys) * monkeys[b].yell(monkeys),
            MonkeIntruction::Div(a, b) => monkeys[a].yell(monkeys) / monkeys[b].yell(monkeys),
        }
    }
}
#[derive(Debug, Clone)]
enum MonkeExpr {
    Human,
    Const(isize),
    Sum(Rc<MonkeExpr>, Rc<MonkeExpr>),
    Sub(Rc<MonkeExpr>, Rc<MonkeExpr>),
    Mul(Rc<MonkeExpr>, Rc<MonkeExpr>),
    Div(Rc<MonkeExpr>, Rc<MonkeExpr>),
}
impl MonkeExpr {
    fn extract<'inp>(
        name: &'inp str,
        instrs: &mut HashMap<&'inp str, MonkeIntruction<'inp>>,
        exprs: &mut HashMap<&'inp str, Rc<MonkeExpr>>,
    ) -> Rc<MonkeExpr> {
        if let Some(name) = exprs.get(name) {
            name.clone()
        } else if name == "humn" {
            if !exprs.contains_key("humn") {
                exprs.insert("humn", Rc::new(MonkeExpr::Human));
            }
            exprs.get("humn").unwrap().clone()
        } else {
            let res = Rc::new(
                match instrs.remove(name).expect(&format!("Missing monke {name}")) {
                    MonkeIntruction::Const(a) => Self::Const(a),
                    MonkeIntruction::Sum(a, b) => Self::Sum(
                        MonkeExpr::extract(a, instrs, exprs),
                        MonkeExpr::extract(b, instrs, exprs),
                    ),
                    MonkeIntruction::Sub(a, b) => Self::Sub(
                        MonkeExpr::extract(a, instrs, exprs),
                        MonkeExpr::extract(b, instrs, exprs),
                    ),
                    MonkeIntruction::Mul(a, b) => Self::Mul(
                        MonkeExpr::extract(a, instrs, exprs),
                        MonkeExpr::extract(b, instrs, exprs),
                    ),
                    MonkeIntruction::Div(a, b) => Self::Div(
                        MonkeExpr::extract(a, instrs, exprs),
                        MonkeExpr::extract(b, instrs, exprs),
                    ),
                },
            );
            exprs.insert(name, res.clone());
            res
        }
    }

    fn const_propagation(self: Rc<Self>) -> Rc<Self> {
        use MonkeExpr::*;
        match self.as_ref() {
            Human | Const(_) => self,
            Sum(a, b) => {
                let (a, b) = (a.clone().const_propagation(), b.clone().const_propagation());
                match (a.as_ref(), b.as_ref()) {
                    (_, Const(0)) => a,
                    (Const(0), _) => b,
                    (Const(a), Const(b)) => Rc::new(Const(a + b)),
                    (_, _) => Rc::new(Sum(a, b)),
                }
            }
            Sub(a, b) => Rc::new(Sum(a.clone(), Rc::new(Mul(b.clone(), Rc::new(Const(-1))))))
                .const_propagation(),
            Mul(a, b) => {
                let (a, b) = (a.clone().const_propagation(), b.clone().const_propagation());
                match (a.as_ref(), b.as_ref()) {
                    (_, Const(1)) => a,
                    (Const(1), _) => b,
                    (_, Const(0)) => b,
                    (Const(0), _) => a,
                    (Const(a), Const(b)) => Rc::new(Const(a * b)),
                    (_, _) => Rc::new(Mul(a, b)),
                }
            }
            Div(a, b) => {
                let (a, b) = (a.clone().const_propagation(), b.clone().const_propagation());
                match (a.as_ref(), b.as_ref()) {
                    (_, Const(1)) => a,
                    (_, Const(0)) => panic!("Division by 0"),
                    (Const(0), _) => a,
                    (Const(a), Const(b)) => {
                        assert!(a % b == 0);
                        Rc::new(Const(a / b))
                    }
                    (_, _) => Rc::new(Div(a, b)),
                }
            }
        }
    }
}
impl Display for MonkeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeExpr::Human => write!(f, "x"),
            MonkeExpr::Const(a) => write!(f, "{a}"),
            MonkeExpr::Sum(a, b) => write!(f, "({a}) + ({b})"),
            MonkeExpr::Sub(a, b) => write!(f, "({a}) - ({b})"),
            MonkeExpr::Mul(a, b) => write!(f, "({a}) * ({b})"),
            MonkeExpr::Div(a, b) => write!(f, "({a}) / ({b})"),
        }
    }
}

fn monke_name(name: &str) -> &str {
    let name = name.trim();
    if name.len() == 4 && name.chars().all(|ch| ch.is_alphabetic()) {
        name
    } else {
        panic!("{name} is not a monke name")
    }
}

fn parse_name_and_op<'a, 'b>((name, op): (&'a str, &'b str)) -> (&'a str, MonkeIntruction<'b>) {
    let name = monke_name(name);
    let monke = if let Some((a, b)) = op.split_once('+') {
        MonkeIntruction::Sum(monke_name(a), monke_name(b))
    } else if let Some((a, b)) = op.split_once('-') {
        MonkeIntruction::Sub(monke_name(a), monke_name(b))
    } else if let Some((a, b)) = op.split_once('*') {
        MonkeIntruction::Mul(monke_name(a), monke_name(b))
    } else if let Some((a, b)) = op.split_once('/') {
        MonkeIntruction::Div(monke_name(a), monke_name(b))
    } else {
        MonkeIntruction::Const(op.trim().parse().unwrap())
    };
    (name, monke)
}

fn parse_input(input: &str) -> HashMap<&str, MonkeIntruction> {
    input
        .trim()
        .lines()
        .map(|line| parse_name_and_op(line.split_once(':').expect("Missing :")))
        .collect()
}
fn root_eq<'a>(mut instrs: HashMap<&'a str, MonkeIntruction<'a>>) -> Rc<MonkeExpr> {
    use MonkeIntruction::*;
    let (a, b) = match instrs.remove("root").expect("Missing Root Eq") {
        Sum(a, b) | Mul(a, b) | Div(a, b) | Sub(a, b) => (a, b),
        Const(_) => panic!("root is not a two member operation"),
    };
    let mut exprs = HashMap::new();

    let lhs = MonkeExpr::extract(a, &mut instrs, &mut exprs);
    let rhs = MonkeExpr::extract(b, &mut instrs, &mut exprs);
    Rc::new(MonkeExpr::Sub(lhs, rhs))
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MonkePoly(Vec<isize>);

impl MonkePoly {
    fn degree(&self) -> usize {
        self.0.len().saturating_sub(1)
    }
    fn coeff(&self, pow: usize) -> isize {
        if pow <= self.degree() {
            self.0[pow]
        } else {
            0
        }
    }
    fn collect(self) -> (Self, usize) {
        if self.0.len() == 0 {
            return (self, 0);
        }
        if self.0.len() == 1 {
            return (Self(vec![self.0[0].signum()]), self.0[0].abs() as usize);
        }
        let gcd = self.0[1..]
            .iter()
            .fold(self.0[0].abs() as usize, |gcd, &v| {
                if v == 0 {
                    return gcd;
                }
                let v = v.abs() as usize;

                calc_gcd(gcd, v)
            });
        (
            Self(self.0.into_iter().map(|v| v / gcd as isize).collect()),
            gcd,
        )
    }
}

fn calc_gcd(a: usize, b: usize) -> usize {
    let mut max = a.max(b);
    let mut min = a.min(b);
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl Display for MonkePoly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (1..self.0.len()).rev() {
            write!(f, "{}x^{} + ", self.0[i], i)?
        }
        write!(f, "{}", self.0.get(0).unwrap_or(&0))
    }
}

impl From<isize> for MonkePoly {
    fn from(value: isize) -> Self {
        if value != 0 {
            MonkePoly(vec![value])
        } else {
            MonkePoly(vec![])
        }
    }
}

impl Add<MonkePoly> for MonkePoly {
    type Output = MonkePoly;

    fn add(self, rhs: MonkePoly) -> Self::Output {
        let mut coeffs = self.0;
        coeffs.resize(coeffs.len().max(rhs.0.len()), 0);
        for (i, a) in rhs.0.into_iter().enumerate() {
            coeffs[i] += a;
        }
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        MonkePoly(coeffs)
    }
}
impl Add<isize> for MonkePoly {
    type Output = MonkePoly;

    fn add(mut self, rhs: isize) -> Self::Output {
        if self.0.len() == 0 {
            self.0.resize(1, 0);
        }
        self.0[0] += rhs;
        if self.0[0] != 0 {
            self
        } else {
            MonkePoly(vec![])
        }
    }
}

impl Sub<MonkePoly> for MonkePoly {
    type Output = MonkePoly;

    fn sub(self, rhs: MonkePoly) -> Self::Output {
        let mut coeffs = self.0;
        coeffs.resize(coeffs.len().max(rhs.0.len()), 0);
        for (i, a) in rhs.0.into_iter().enumerate() {
            coeffs[i] -= a;
        }
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        MonkePoly(coeffs)
    }
}
impl Sub<isize> for MonkePoly {
    type Output = MonkePoly;

    fn sub(mut self, rhs: isize) -> Self::Output {
        if self.0.len() == 0 {
            self.0.resize(1, 0);
        }
        self.0[0] -= rhs;
        if self.0[0] != 0 {
            self
        } else {
            MonkePoly(vec![])
        }
    }
}

impl Mul<MonkePoly> for MonkePoly {
    type Output = MonkePoly;
    fn mul(self, rhs: MonkePoly) -> Self::Output {
        if self.0.len() == 0 {
            return self;
        }
        if rhs.0.len() == 0 {
            return rhs;
        }

        let mut coeffs = vec![0; (self.0.len() - 1) + (rhs.0.len() - 1) + 1];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in rhs.0.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }

        MonkePoly(coeffs)
    }
}
impl Mul<isize> for MonkePoly {
    type Output = MonkePoly;
    fn mul(self, rhs: isize) -> Self::Output {
        if self.0.len() == 0 {
            return self;
        }
        if rhs == 0 {
            return MonkePoly(vec![]);
        }

        MonkePoly(self.0.into_iter().map(|c| c * rhs).collect())
    }
}

impl Div<MonkePoly> for MonkePoly {
    type Output = MonkeFract;

    fn div(self, rhs: MonkePoly) -> Self::Output {
        MonkeFract {
            num: self,
            frac: rhs,
        }
    }
}
impl Div<isize> for MonkePoly {
    type Output = MonkeFract;

    fn div(self, rhs: isize) -> Self::Output {
        assert!(rhs != 0, "Division by 0");
        MonkeFract {
            num: self,
            frac: MonkePoly(vec![rhs]),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MonkeFract {
    num: MonkePoly,
    frac: MonkePoly,
}
impl MonkeFract {
    // reduce the fraction as needed
    fn reduce(self) -> MonkeFract {
        let Self { num, frac } = self; // decompose
        let (num, num_gcd) = num.collect();
        let (frac, frac_gcd) = frac.collect();
        let common = calc_gcd(num_gcd, frac_gcd);
        Self {
            num: num * (num_gcd / common) as isize,
            frac: frac * (frac_gcd / common) as isize,
        }
    }
}

impl Display for MonkeFract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) / ({})", self.num, self.frac)
    }
}

impl From<Rc<MonkeExpr>> for MonkeFract {
    fn from(value: Rc<MonkeExpr>) -> Self {
        match value.as_ref() {
            MonkeExpr::Human => Self::from(MonkePoly(vec![0, 1])),
            MonkeExpr::Const(a) => Self::from(*a),
            MonkeExpr::Sum(a, b) => Self::from(a.clone()) + Self::from(b.clone()),
            MonkeExpr::Sub(a, b) => Self::from(a.clone()) - Self::from(b.clone()),
            MonkeExpr::Mul(a, b) => Self::from(a.clone()) * Self::from(b.clone()),
            MonkeExpr::Div(a, b) => Self::from(a.clone()) / Self::from(b.clone()),
        }
    }
}

impl From<MonkePoly> for MonkeFract {
    fn from(value: MonkePoly) -> Self {
        Self {
            num: value,
            frac: MonkePoly::from(1),
        }
    }
}
impl From<isize> for MonkeFract {
    fn from(value: isize) -> Self {
        Self {
            num: MonkePoly::from(value),
            frac: MonkePoly::from(1),
        }
    }
}

impl Add<MonkeFract> for MonkeFract {
    type Output = MonkeFract;

    fn add(self, rhs: MonkeFract) -> Self::Output {
        MonkeFract {
            num: self.num * rhs.frac.clone() + rhs.num * self.frac.clone(),
            frac: self.frac * rhs.frac,
        }
        .reduce()
    }
}
impl Add<MonkePoly> for MonkeFract {
    type Output = MonkeFract;

    fn add(self, rhs: MonkePoly) -> Self::Output {
        MonkeFract {
            num: self.num + rhs * self.frac.clone(),
            frac: self.frac,
        }
        .reduce()
    }
}
impl Add<isize> for MonkeFract {
    type Output = MonkeFract;

    fn add(self, rhs: isize) -> Self::Output {
        MonkeFract {
            num: self.num + self.frac.clone() * rhs,
            frac: self.frac,
        }
        .reduce()
    }
}

impl Sub<MonkeFract> for MonkeFract {
    type Output = MonkeFract;

    fn sub(self, rhs: MonkeFract) -> Self::Output {
        MonkeFract {
            num: self.num * rhs.frac.clone() - rhs.num * self.frac.clone(),
            frac: self.frac * rhs.frac,
        }
        .reduce()
    }
}
impl Sub<MonkePoly> for MonkeFract {
    type Output = MonkeFract;

    fn sub(self, rhs: MonkePoly) -> Self::Output {
        MonkeFract {
            num: self.num - rhs * self.frac.clone(),
            frac: self.frac,
        }
        .reduce()
    }
}
impl Sub<isize> for MonkeFract {
    type Output = MonkeFract;

    fn sub(self, rhs: isize) -> Self::Output {
        MonkeFract {
            num: self.num - self.frac.clone() * rhs,
            frac: self.frac,
        }
        .reduce()
    }
}

impl Mul<MonkeFract> for MonkeFract {
    type Output = MonkeFract;
    fn mul(self, rhs: MonkeFract) -> Self::Output {
        Self {
            num: self.num * rhs.num,
            frac: self.frac * rhs.frac,
        }
        .reduce()
    }
}
impl Mul<MonkePoly> for MonkeFract {
    type Output = MonkeFract;
    fn mul(self, rhs: MonkePoly) -> Self::Output {
        Self {
            num: self.num * rhs,
            frac: self.frac,
        }
        .reduce()
    }
}
impl Mul<isize> for MonkeFract {
    type Output = MonkeFract;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            num: self.num * rhs,
            frac: self.frac,
        }
        .reduce()
    }
}

impl Div<MonkeFract> for MonkeFract {
    type Output = MonkeFract;
    fn div(self, rhs: MonkeFract) -> Self::Output {
        Self {
            num: self.num * rhs.frac,
            frac: self.frac * rhs.num,
        }
        .reduce()
    }
}
impl Div<MonkePoly> for MonkeFract {
    type Output = MonkeFract;
    fn div(self, rhs: MonkePoly) -> Self::Output {
        Self {
            num: self.num,
            frac: self.frac * rhs,
        }
        .reduce()
    }
}
impl Div<isize> for MonkeFract {
    type Output = MonkeFract;
    fn div(self, rhs: isize) -> Self::Output {
        Self {
            num: self.num,
            frac: self.frac * rhs,
        }
        .reduce()
    }
}

pub fn part1(input: &str) -> isize {
    let monkeys = parse_input(input);

    monkeys["root"].yell(&monkeys)
}

pub fn part2(input: &str) -> isize {
    let monkeys = parse_input(input);
    let eq = MonkeFract::from(root_eq(monkeys).const_propagation());

    // print!("{eq}");

    if eq.frac == MonkePoly::from(0) {
        panic!("Equation is impossible");
    }

    // find zeroes
    match (eq.num.degree(), eq.frac.degree()) {
        (0, 0) => panic!("Equation is a constant expression"),
        (1, 0) => {
            let a = -eq.num.coeff(1);
            let b = eq.num.coeff(0);
            if b % a != 0 {
                panic!("Solution is not whole");
            }
            b / a
        }
        _ => unimplemented!("Solutions of other degrees are still unimplemented"),
    }
}
