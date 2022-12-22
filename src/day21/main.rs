use shared::puzzle_input;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
enum Expr<'a> {
    Literal(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Match(&'a str, &'a str),
    Humn,
}
impl<'a> From<&'a str> for Expr<'a> {
    fn from(s: &'a str) -> Self {
        if s.contains('+') {
            let (a, b) = s.split_once(" + ").unwrap();
            Expr::Add(a, b)
        } else if s.contains('-') {
            let (a, b) = s.split_once(" - ").unwrap();
            Expr::Sub(a, b)
        } else if s.contains('*') {
            let (a, b) = s.split_once(" * ").unwrap();
            Expr::Mul(a, b)
        } else if s.contains('/') {
            let (a, b) = s.split_once(" / ").unwrap();
            Expr::Div(a, b)
        } else {
            Expr::Literal(s.parse::<i64>().unwrap())
        }
    }
}
impl<'a> Expr<'a> {
    pub fn eval(&self, name: &'a str, env: &mut BTreeMap<&'a str, Self>) -> Option<i64> {
        match self {
            Expr::Literal(x) => Some(*x),
            Expr::Add(a_name, b_name)
            | Expr::Sub(a_name, b_name)
            | Expr::Mul(a_name, b_name)
            | Expr::Div(a_name, b_name) => self._eval(name, a_name, b_name, env),
            Expr::Match(_, _) | Expr::Humn => None,
        }
    }
    fn _eval(
        &self,
        name: &'a str,
        a_name: &'a str,
        b_name: &'a str,
        env: &mut BTreeMap<&'a str, Self>,
    ) -> Option<i64> {
        let a_expr = *env.get(a_name).unwrap();
        let b_expr = *env.get(b_name).unwrap();
        let a = a_expr.eval(a_name, env)?;
        let b = b_expr.eval(b_name, env)?;
        let x = self.op(a, b);
        env.insert(name, Expr::Literal(x));
        Some(x)
    }
    pub fn solve(&self, env: &mut BTreeMap<&'a str, Self>) -> i64 {
        if let Expr::Match(a_name, b_name) = self {
            let a_expr = *env.get(a_name).unwrap();
            let b_expr = *env.get(b_name).unwrap();
            if let Some(x) = a_expr.eval(a_name, env) {
                return b_expr.set_equal(x, env);
            } else if let Some(x) = b_expr.eval(b_name, env) {
                return a_expr.set_equal(x, env);
            }
        }
        unreachable!()
    }
    fn solve_lhs(&self, rhs: i64, result: i64) -> i64 {
        match self {
            Expr::Add(_, _) => result - rhs,
            Expr::Sub(_, _) => result + rhs,
            Expr::Mul(_, _) => result / rhs,
            Expr::Div(_, _) => result * rhs,
            _ => unreachable!(),
        }
    }
    fn solve_rhs(&self, lhs: i64, result: i64) -> i64 {
        match self {
            Expr::Add(_, _) => result - lhs,
            Expr::Sub(_, _) => lhs - result,
            Expr::Mul(_, _) => result / lhs,
            Expr::Div(_, _) => lhs / result,
            _ => unreachable!(),
        }
    }
    fn set_equal(&self, value: i64, env: &mut BTreeMap<&'a str, Self>) -> i64 {
        match self {
            Expr::Humn => value,
            Expr::Add(a_name, b_name)
            | Expr::Sub(a_name, b_name)
            | Expr::Mul(a_name, b_name)
            | Expr::Div(a_name, b_name) => {
                let a_expr = *env.get(a_name).unwrap();
                let b_expr = *env.get(b_name).unwrap();
                if let Some(a) = a_expr.eval(a_name, env) {
                    return b_expr.set_equal(self.solve_rhs(a, value), env);
                } else if let Some(b) = b_expr.eval(b_name, env) {
                    return a_expr.set_equal(self.solve_lhs(b, value), env);
                }
                unreachable!();
            }
            _ => unreachable!(),
        }
    }
    fn op(&self, a: i64, b: i64) -> i64 {
        match self {
            Expr::Add(_, _) => a + b,
            Expr::Sub(_, _) => a - b,
            Expr::Mul(_, _) => a * b,
            Expr::Div(_, _) => a / b,
            _ => unreachable!(),
        }
    }
}

pub fn main() {
    let input = puzzle_input!();
    let mut exprs = input
        .split('\n')
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            (name, rest.into())
        })
        .collect::<BTreeMap<&str, Expr>>();
    let p1 = exprs["root"]
        .clone()
        .eval("root", &mut exprs.clone())
        .unwrap();

    if let Expr::Add(a, b) = exprs.get("root").unwrap() {
        exprs.insert("root", Expr::Match(a, b));
    }
    exprs.insert("humn", Expr::Humn);
    let p2 = exprs["root"].clone().solve(&mut exprs);

    // 158731561459602, 3769668716709
    println!("part 1: {}\npart 2: {}", p1, p2);
}
