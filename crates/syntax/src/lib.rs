use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    LetKw,
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comment,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
        }
    }
}

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::FnKw => "‘fn’",
            SyntaxKind::LetKw => "‘let’",
            SyntaxKind::Ident => "identifier",
            SyntaxKind::Number => "number",
            SyntaxKind::Plus => "‘+’",
            SyntaxKind::Minus => "‘-’",
            SyntaxKind::Star => "‘*’",
            SyntaxKind::Slash => "‘/’",
            SyntaxKind::Equals => "‘=’",
            SyntaxKind::LParen => "‘(’",
            SyntaxKind::RParen => "‘)’",
            SyntaxKind::LBrace => "‘{’",
            SyntaxKind::RBrace => "‘}’",
            SyntaxKind::Comment => "comment",
            _ => unreachable!(),
        })
    }
}

pub type SyntaxNode = rowan::SyntaxNode<EldiroLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EldiroLanguage {}

impl rowan::Language for EldiroLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
