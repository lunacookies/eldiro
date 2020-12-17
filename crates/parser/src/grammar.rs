mod expr;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    expr::expr(p);

    m.complete(p, SyntaxKind::Root)
}
