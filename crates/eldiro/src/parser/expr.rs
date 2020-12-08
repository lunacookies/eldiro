use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        Some(SyntaxKind::Minus) => {
            let op = PrefixOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            // Eat the operator’s token.
            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::PrefixExpr);
            expr_binding_power(p, right_binding_power);
            p.finish_node();
        }
        Some(SyntaxKind::LParen) => {
            p.bump();
            expr_binding_power(p, 0);

            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.bump();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        // Eat the operator’s token.
        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Number@0..3 "999"
  Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Number@1..4 "123"
  Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Ident@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  BinaryExpr@0..3
    Number@0..1 "1"
    Plus@1..2 "+"
    Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  BinaryExpr@0..7
    BinaryExpr@0..5
      BinaryExpr@0..3
        Number@0..1 "1"
        Plus@1..2 "+"
        Number@2..3 "2"
      Plus@3..4 "+"
      Number@4..5 "3"
    Plus@5..6 "+"
    Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  BinaryExpr@0..7
    BinaryExpr@0..5
      Number@0..1 "1"
      Plus@1..2 "+"
      BinaryExpr@2..5
        Number@2..3 "2"
        Star@3..4 "*"
        Number@4..5 "3"
    Minus@5..6 "-"
    Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  BinaryExpr@1..12
    Number@1..2 "1"
    Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    BinaryExpr@7..12
      Number@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Number@10..11 "3"
      Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 # Add one
  + 10 # Add ten",
            expect![[r##"
Root@0..35
  Whitespace@0..1 "\n"
  BinaryExpr@1..35
    BinaryExpr@1..21
      Number@1..2 "1"
      Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Number@7..8 "1"
      Whitespace@8..9 " "
      Comment@9..18 "# Add one"
      Whitespace@18..21 "\n  "
    Plus@21..22 "+"
    Whitespace@22..23 " "
    Number@23..25 "10"
    Whitespace@25..26 " "
    Comment@26..35 "# Add ten""##]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Number@1..3 "10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
Root@0..6
  BinaryExpr@0..6
    PrefixExpr@0..3
      Minus@0..1 "-"
      Number@1..3 "20"
    Plus@3..4 "+"
    Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  LParen@0..1 "("
  LParen@1..2 "("
  LParen@2..3 "("
  LParen@3..4 "("
  LParen@4..5 "("
  LParen@5..6 "("
  Number@6..8 "10"
  RParen@8..9 ")"
  RParen@9..10 ")"
  RParen@10..11 ")"
  RParen@11..12 ")"
  RParen@12..13 ")"
  RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  BinaryExpr@0..7
    Number@0..1 "5"
    Star@1..2 "*"
    LParen@2..3 "("
    BinaryExpr@3..6
      Number@3..4 "2"
      Plus@4..5 "+"
      Number@5..6 "1"
    RParen@6..7 ")""#]],
        );
    }
}
