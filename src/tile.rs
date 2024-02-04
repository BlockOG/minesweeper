use crate::number::Number;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tile {
    Empty,
    Mine,
    Number(Number),
}
