use eframe::epaint::Pos2;
use serde_derive::{Deserialize, Serialize};

use crate::{get_size, BORDER_HEIGHT, BORDER_WIDTH, FIELD_SIZE, NUMBER_HEIGHT, NUMBER_MARGIN};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Position {
    pub(crate) fn new(x: usize, y: usize) -> Option<Self> {
        if x < get_size().0 && y < get_size().1 {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self {
            x: index % get_size().0,
            y: index / get_size().0,
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.y * get_size().0 + self.x
    }

    pub(crate) fn coordinates(&self) -> (f32, f32) {
        (
            BORDER_WIDTH + self.x as f32 * FIELD_SIZE + FIELD_SIZE / 2.0,
            BORDER_HEIGHT * 2.0
                + NUMBER_MARGIN * 2.0
                + NUMBER_HEIGHT
                + self.y as f32 * FIELD_SIZE
                + FIELD_SIZE / 2.0,
        )
    }

    pub(crate) fn from_mouse(mut pos: Pos2) -> Option<Self> {
        pos.x -= BORDER_WIDTH;
        pos.y -= BORDER_HEIGHT * 2.0 + NUMBER_MARGIN * 2.0 + NUMBER_HEIGHT;

        if pos.x < 0.0 || pos.y < 0.0 {
            None
        } else {
            Self::new((pos.x / FIELD_SIZE) as usize, (pos.y / FIELD_SIZE) as usize)
        }
    }

    pub(crate) fn is_near(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    pub(crate) fn neighbors(&self) -> Vec<Self> {
        (self.x.saturating_sub(1)..=self.x + 1)
            .map(|x| (self.y.saturating_sub(1)..=self.y + 1).map(move |y| Self::new(x, y)))
            .flatten()
            .flatten()
            .filter(|pos| pos != self)
            .collect()
    }
}
