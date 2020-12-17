mod event;
mod expr;
mod marker;
mod sink;
mod source;

use event::Event;
use expr::expr;
use lexer::{Lexer, Token};
use marker::Marker;
use rowan::GreenNode;
use sink::Sink;
use source::Source;
use syntax::{SyntaxKind, SyntaxNode};

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    fn new(tokens: &'t [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        );
    }
}
