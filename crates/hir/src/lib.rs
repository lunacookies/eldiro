use smol_str::SmolStr;
use syntax::SyntaxKind;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: SmolStr, value: Expr },
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Self>,
    },
    VariableRef {
        var: SmolStr,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> impl Iterator<Item = Stmt> {
    ast.stmts().filter_map(Stmt::lower)
}

impl Stmt {
    fn lower(ast: ast::Stmt) -> Option<Self> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Self::VariableDef {
                name: ast.name()?.text().clone(),
                value: Expr::lower(ast.value()),
            },
            ast::Stmt::Expr(ast) => Self::Expr(Expr::lower(Some(ast))),
        };

        Some(result)
    }
}

impl Expr {
    fn lower(ast: Option<ast::Expr>) -> Self {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => Self::lower_binary(ast),
                ast::Expr::Literal(ast) => Self::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => Expr::lower(ast.expr()),
                ast::Expr::UnaryExpr(ast) => Self::lower_unary(ast),
                ast::Expr::VariableRef(ast) => Self::VariableRef { var: ast.name() },
            }
        } else {
            Self::Missing
        }
    }

    fn lower_binary(ast: ast::BinaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        Self::Binary {
            op,
            lhs: Box::new(Expr::lower(ast.lhs())),
            rhs: Box::new(Expr::lower(ast.rhs())),
        }
    }

    fn lower_unary(ast: ast::UnaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        Self::Unary {
            op,
            expr: Box::new(Expr::lower(ast.expr())),
        }
    }
}
