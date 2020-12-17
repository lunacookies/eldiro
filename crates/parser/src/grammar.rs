mod expr;
mod stmt;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    stmt::stmt(p);

    m.complete(p, SyntaxKind::Root)
}
