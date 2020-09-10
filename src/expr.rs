use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };

        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        };

        Val::Number(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), ("", Op::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), ("", Op::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), ("", Op::Div));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
            ),
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                "",
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
            ),
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(),
            Val::Number(20),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(),
            Val::Number(-4),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mul,
            }
            .eval(),
            Val::Number(30),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(),
            Val::Number(10),
        );
    }
}
