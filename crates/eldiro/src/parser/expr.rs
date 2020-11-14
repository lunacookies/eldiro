use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        _ => {}
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
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Ident@0..7 "counter""#]],
        );
    }
}
