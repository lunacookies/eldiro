use syntax::SyntaxKind;

#[derive(Debug, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
}
