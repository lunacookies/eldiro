use crate::binding_def::BindingDef;
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    BindingDef(BindingDef),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Expr::new(s)
            .map(|(s, expr)| (s, Self::Expr(expr)))
            .or_else(|_| {
                BindingDef::new(s).map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

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
}
