use crate::lexer::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
}
