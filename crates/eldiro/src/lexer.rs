use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

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
}
