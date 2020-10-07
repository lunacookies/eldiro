use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expr::Expr;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            }
            Self::Expr(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 10"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Stmt::new("1+1"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Add,
                }),
            )),
        );
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Stmt::BindingDef(BindingDef {
                name: "whatever".to_string(),
                val: Expr::Number(Number(-10)),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(
            Stmt::Expr(Expr::Number(Number(5))).eval(&mut Env::default()),
            Ok(Val::Number(5)),
        );
    }
}
