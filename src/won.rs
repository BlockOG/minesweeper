use crate::position::Position;

#[derive(Clone, Copy)]
pub(crate) enum Won {
    None,
    Won,
    Lost(Position),
}

impl Won {
    pub(crate) fn still_playing(&self) -> bool {
        matches!(self, Self::None)
    }

    pub(crate) fn win(&mut self) {
        *self = Self::Won;
    }

    pub(crate) fn lose(&mut self, position: Position) {
        *self = Self::Lost(position);
    }
}
