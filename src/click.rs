use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

use crate::position::Position;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) enum Click {
    Reveal(Position, Duration),
    Flag(Position, Duration),
}

impl Click {
    pub(crate) fn position(&self) -> Position {
        match self {
            Self::Reveal(pos, _) | Self::Flag(pos, _) => *pos,
        }
    }

    pub(crate) fn duration(&self) -> Duration {
        match self {
            Self::Reveal(_, duration) | Self::Flag(_, duration) => *duration,
        }
    }
}
