use crate::lexer::{Lexeme, SyntaxKind};

pub(super) struct Source<'l, 'input> {
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
}

impl<'l, 'input> Source<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self { lexemes, cursor: 0 }
    }

    pub(super) fn next_lexeme(&mut self) -> Option<&'l Lexeme<'input>> {
        self.eat_whitespace();

        let lexeme = self.lexemes.get(self.cursor)?;
        self.cursor += 1;

        Some(lexeme)
    }

    pub(super) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_whitespace();
        self.peek_kind_raw()
    }

    fn eat_whitespace(&mut self) {
        while self.peek_kind_raw() == Some(SyntaxKind::Whitespace) {
            self.cursor += 1;
        }
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }
}
