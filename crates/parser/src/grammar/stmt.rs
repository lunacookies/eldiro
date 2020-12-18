use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    match p.peek() {
        Some(SyntaxKind::LetKw) => Some(variable_def(p)),
        _ => expr::expr(p),
    }
}

fn variable_def(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::LetKw));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Ident);
    p.expect(SyntaxKind::Equals);

    expr::expr(p);

    m.complete(p, SyntaxKind::VariableDef)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "let foo = bar",
            expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..13
      Ident@10..13 "bar""#]],
        );
    }
}
