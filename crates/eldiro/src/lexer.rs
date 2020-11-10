use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_spaces() {
        let mut lexer = SyntaxKind::lexer("   ");

        assert_eq!(lexer.next(), Some(SyntaxKind::Whitespace));
        assert_eq!(lexer.slice(), "   ");
    }

    #[test]
    fn lex_fn_keyword() {
        let mut lexer = SyntaxKind::lexer("fn");

        assert_eq!(lexer.next(), Some(SyntaxKind::FnKw));
        assert_eq!(lexer.slice(), "fn");
    }
}
