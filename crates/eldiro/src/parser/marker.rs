use super::event::Event;
use super::Parser;
use crate::lexer::SyntaxKind;
use drop_bomb::DropBomb;

pub(super) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(super) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub(super) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) {
        self.bomb.defuse();

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode { kind };

        p.events.push(Event::FinishNode);
    }
}
