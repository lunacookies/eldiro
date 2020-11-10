use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("let")]
    LetKw,

    #[regex("[a-z0-9]+")]
    Ident,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_eq!(lexer.next(), Some(kind));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }
}
